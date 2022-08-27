<script type="ts">
    import type {WorkerMessageType} from "./lib/WorkerMessage";
    import {WorkerMessage} from "./lib/WorkerMessage";
    import {storeParams, restoreParams} from "./lib/localstorage";
    import type {DownloadLinkProps} from "./lib/download";
    import {getDownloadLinkProps} from "./lib/download";

    const worker = new Worker(new URL('./lib/worker.ts', import.meta.url), {
        type: "module"
    });
    let workerReady = false;
    let queued: WorkerMessageType | undefined = undefined;

    function requestCodegen(message: WorkerMessageType) {
        if (workerReady) {
            worker.postMessage(message);
            workerReady = false;
            // $('target-wrapper').classList.add('is-loading');
        } else {
            queued = message;
        }
    }

    const storedParams = restoreParams();
    let input = storedParams.input || '';
    let output = '';
    let outputMode = storedParams.options?.output_mode || 'kotlin/jackson';
    let typeName = storedParams.typename || 'MyRoot';
    let propertyNameFormat = storedParams.options?.property_name_format || '';
    let importStyle = storedParams.options?.import_style || 'assume_existing';
    let unwrapPath = storedParams.options?.unwrap || '';
    let collectAdditional = storedParams.options?.collect_additional || false;
    let extraOptions = storedParams.extraoptions || '';

    let downloadLink: DownloadLinkProps | undefined = undefined;

    const conditionalOptions = {
        propertynameformat: ['rust', 'kotlin/jackson', 'python'],
        importstyle: ['rust', 'kotlin/jackson', 'kotlin/kotlinx', 'python'],
        collectadditional: ['rust', 'kotlin/jackson'],
    };

    worker.onmessage = messageEvent => {
        const message: WorkerMessageType = messageEvent.data;
        if (message.type === WorkerMessage.CODEGEN_COMPLETE) {
            output = message.result.trim();
            // TODO: auto-adjust output height
            // target.style.height = "10px";
            // target.style.height = (target.scrollHeight + 5) + "px";

            downloadLink = getDownloadLinkProps(output, message.typename, message.options['output_mode'])
        } else if (message.type === WorkerMessage.LOAD_FILE_COMPLETE) {
            // $('large-file-spinner').classList.add('is-invisible');
            // $('clear-input-button').classList.remove('is-invisible');
        } else if (message.type === WorkerMessage.WASM_READY) {
            // no action needed
            console.log('Worker ready')
        } else {
            console.warn("Unknown worker message ", messageEvent);
        }

        // If we got a message (any message) from the worker, it is ready
        // (we don't have any kind of queue at the worker side)
        workerReady = true;
        if (queued) {
            requestCodegen(queued);
            queued = undefined;
        }
    }

    const render = () => {
        const options = {
            output_mode: outputMode,
            property_name_format: propertyNameFormat,
            import_style: importStyle,
            unwrap: unwrapPath,
            collect_additional: collectAdditional,
        };
        storeParams({
            typename: typeName,
            input: (input.length < 1000000) ? input : '',
            options,
            extraoptions: extraOptions,
        });

        let parsedExtraOptions = {};
        try {
            parsedExtraOptions = JSON.parse(extraOptions);
        } catch (e) {
            // TODO
        }

        const combinedOptions = Object.assign({}, options, parsedExtraOptions);

        const message: WorkerMessageType = {
            type: WorkerMessage.CODEGEN,
            typename: typeName,
            input: input || '{}',
            options: combinedOptions,
        };
        requestCodegen(message);
    }
    render();
</script>

<input bind:value={typeName} on:keyup={render}>

<textarea bind:value={input} on:keyup={render}></textarea>

<select bind:value={outputMode} on:change={render}>
    <option value="rust">Rust</option>
    <option value="typescript">Typescript</option>
    <option value="typescript/typealias">Typescript (single typealias)</option>
    <option value="kotlin/jackson">Kotlin (Jackson)</option>
    <option value="kotlin/kotlinx">Kotlin (kotlinx.serialization)</option>
    <option value="python">Python (pydantic)</option>
    <option value="json_schema">JSON Schema</option>
    <option value="shape">Shape (internal representation)</option>
</select>

{#if conditionalOptions.propertynameformat.includes(outputMode)}
    <select bind:value={propertyNameFormat} on:change={render}>
        <option value=""></option>
        <option value="PascalCase">PascalCase</option>
        <option value="camelCase">camelCase</option>
        <option value="snake_case">snake_case</option>
        <option value="SCREAMING_SNAKE_CASE">SCREAMING_SNAKE_CASE</option>
        <option value="kebab-case">kebab-case</option>
        <option value="SCREAMING-KEBAB-CASE">SCREAMING-KEBAB-CASE</option>
        <option value="UPPERCASE">UPPERCASE</option>
    </select>
{/if}

{#if conditionalOptions.importstyle.includes(outputMode)}
    <select bind:value={importStyle} on:change={render}>
        <option value="assume_existing">Assume existing imports</option>
        <option value="add_imports">Add needed imports</option>
        <option value="qualified_paths">Use fully qualified paths</option>
    </select>
{/if}

{#if conditionalOptions.collectadditional.includes(outputMode)}
    <input type="checkbox" bind:value={collectAdditional}>
{/if}

<input bind:value={unwrapPath} on:keyup={render}>

<textarea bind:value={extraOptions} on:keyup={render}></textarea>

{#if downloadLink}
    <a href={downloadLink.href} download={downloadLink.download}>Download as file</a>
{/if}
<textarea bind:value={output} readonly></textarea>
