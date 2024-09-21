<script>
    import { onMount } from "svelte";
    import __wbg_init from "../../assets/bindings/rustwasm";
  
    onMount(async () => {
      await __wbg_init();
    });
  </script>
  
  <canvas id="game_canvas" />

  <style>
    canvas {
        position: absolute;
        width: 100vw;
        height: 100vh;
        z-index: -1;
    }
  </style>