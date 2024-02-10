<script>
    let name = 'world';
</script>

<div style="flex-flow: row nowrap;">
  <div class="schedule-lane">
    <div class="task">
      <div class="left-handle"></div>
      <div class="center-handle"></div>
      <div class="right-handle"></div>
    </div>
  </div>
</div>

<style>
    .schedule-lane {
        height: 40px;
        width: calc(40px * 10);
        flex-wrap: nowrap;
        display: flex;
    }
    .task {
        flex-wrap: nowrap;
        display: flex;
        width: calc(40px * 4)
    }
    .center-handle {
        background-color:green;
        height: 40px;
        /* width: calc(40px * 10); */
        flex-grow: 1;
        cursor: grab;
    }
    .left-handle {
        border-radius: 4px 0 0 4px;
        background-color: red;
        height: 40px;
        min-width: 8px;
        width: 8px;
        cursor: grab;
        place-items: end;
    }
    .right-handle {
        border-radius: 0 4px 4px 0;
        background-color: blue;
        height: 40px;
        min-width: 8px;
        width: 8px;
        cursor: grab;
    }
</style>