<script lang="ts">
    let innerWidth = $state(0);
    let innerHeight = $state(0);

    function onKeyPress(e: KeyboardEvent) {

    }
</script>

<svelte:window onkeypress={onKeyPress} bind:innerWidth bind:innerHeight />

<style>
 
</style>

<main>

</main>
