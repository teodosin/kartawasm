<script lang="ts">
    import Game from "../lib/components/Client.svelte";

    function handleClick() {
        console.log("Clicked");
    }

    export function button_is_pressed() {
        console.log("Button is pressed");
    }

</script>

<Game />

<div class="cont">
    

    <h1>Testing stuff for Karta</h1>
    
    <button on:click={handleClick}>
        Clickity
    </button>
    
</div>

<style>
    :global(body) {
        background: #222222;
        margin: 0;
        color: #dddddd;
    }
    .cont {
        height: 100vh;
        max-width: 20rem;
        margin-left: auto;
        margin-right: auto;

        display: flex;
        flex-direction: column;
        align-content: center;

        background: #49494970;
    }
    button {
        cursor: pointer;
    }
</style>