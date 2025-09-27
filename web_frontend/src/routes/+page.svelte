<script lang="ts">
    import { onMount } from "svelte";
    import initWASM, { engineInit, engineSetDimension } from "sudoku_engine";

    let initiated = $state(false);
    let innerWidth = $state(0);
    let innerHeight = $state(0);

    let canvas: HTMLCanvasElement;

    onMount(() => {
        if (!initiated) {
            initWASM().then(() => {
                initiated = true;
                let context = canvas.getContext("2d");
                if (!context) {
                    return;
                }
                engineInit(canvas);
                engineSetDimension(Math.min(innerWidth, innerHeight));
            });
        }
    });
</script>

<svelte:window bind:innerWidth bind:innerHeight />

<style>
    main {
        width: 100vw;
        height: 100vh;
        display: flex;
    }

    canvas {
        margin: auto;
    }
</style>

<main>
    <canvas
        bind:this={canvas}
        width={Math.min(innerHeight, innerWidth)}
        height={Math.min(innerHeight, innerWidth)}
        style={`width: ${Math.min(innerHeight, innerWidth)}px; height: ${Math.min(innerHeight, innerWidth)}px;`}
    >
    </canvas>
</main>
