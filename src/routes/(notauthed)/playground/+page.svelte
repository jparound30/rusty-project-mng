<script>
    import TaskLane from "$lib/TaskLane.svelte";

    let name = 'world';
</script>

<div class="outer">
  <TaskLane/>
  <TaskLane/>
<!--  <div class="tasks">-->

<!--  </div>-->
<!--  <div class="outer-calender">-->
<!--    <div class="calender">-->
<!--      aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa-->
<!--    </div>-->
<!--  </div>-->
</div>

<style>
    .tasks {
        width: calc(200px);
        min-width: calc(200px);
        height: calc(60px * 40);
        background-color: lightblue;
    }
    .outer-calender {
        width: calc(90% - 200px);
        height: calc(60px * 40);
        overflow-x: scroll;
        overflow-y: hidden;
        /* scroll-behavior: ; */
        scrollbar-width: none;
    }
    .calender {
        width: calc(40px * 30);
        height: calc(60px * 40);
        background-color: pink;
    }

    .outer {
        width : 90%;
        height: calc(60px * 40 + 80px);
        overflow-x: scroll;
    }
</style>