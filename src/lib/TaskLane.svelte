<script lang="ts">
    // let selectedPart: string | null = null;
    let start: number;
    let end: number;
    let xPos: number;


    const positionToStyle = (pos: number) => `translateX(${pos}px)`;

    function dragstart(event: DragEvent) {
        if (event?.target instanceof Element) {
            const selectedPart = event.target.classList.contains('left-handle') ? 'start' :
                event.target.classList.contains('right-handle') ? 'end' : 'middle';
            console.log("dragstart  : " + selectedPart)
            event.dataTransfer?.setData("text/plain", selectedPart); // Add this line
            let selectedPart2 = event.dataTransfer?.getData("text/plain");

            console.log("dragstart: selectedPart = " + selectedPart2)

            var click_x = event.pageX ;

            // 要素の位置を取得
            var client_rect = (event.target as Element).getBoundingClientRect() ;
            var position_x = client_rect.left + window.scrollX ;

            // 要素内におけるクリック位置を計算
            xPos = click_x - position_x ;

        } else {
            console.log("dragstart  : OTHER EVENT")
        }
    }

    function drag(event: DragEvent) {
          let selectedPart = event.dataTransfer?.getData("text/plain");

          console.log("drag: selectedPart = " + selectedPart)
          if (selectedPart == 'start') {
              start = event.clientX - xPos;
              console.log("drag start: " + start)
          } else if (selectedPart == 'end') {
              end = event.clientX - xPos;
              console.log("drag end  : " + end)
          } else if (selectedPart == 'middle') {
              // update both start and end keeping the same distance
              const distance = end - start;
              start = event.clientX - xPos;
              end = start + distance;
              console.log("drag middle  : " + start + ", " + end)
          }
          else {
              console.log("drag other" + "(" + selectedPart + ")" + " : " + start + ", " + end)
          }
    }

    function dragend(event: DragEvent) {
        let selectedPart = event.dataTransfer?.getData("text/plain");
        console.log("dragend  : " + selectedPart)
        // reset selected part
        // selectedPart = null;
    }

    function drop(event: DragEvent) {
        let selectedPart = event.dataTransfer?.getData("text/plain");
        console.log("drop  : " + selectedPart)
        var click_x = event.pageX ;

        // 要素の位置を取得
        var client_rect = (event.target as Element).getBoundingClientRect() ;
        var position_x = client_rect.left + window.scrollX ;

        // 要素内におけるクリック位置を計算
        var x = click_x - position_x ;

        if (selectedPart == 'start') {
            start = event.clientX - xPos;
            console.log("drag start: " + start)
        } else if (selectedPart == 'end') {
            end = event.clientX  - xPos;
            console.log("drag end  : " + end)
        } else if (selectedPart == 'middle') {
            // update both start and end keeping the same distance
            const distance = 100;//end - start; TODO 幅はdragstart時点で決まっているのでそれを持っておく
            start = event.clientX  - xPos;
            end = start + distance;
            console.log("drag middle  : " + start + ", " + end)
        }
        // reset selected part
        // selectedPart = null;
        event.preventDefault();
    }

    function dragover(event: DragEvent) {
        console.log("dragover")
        // prevent default to allow drop
        event.preventDefault();
        if (event.dataTransfer != null) {
            event.dataTransfer.dropEffect = "move";
        }
        return true;
    }
    function dragenter(event: DragEvent) {
        console.log("dragenter")
        // prevent default to allow drop
        event.preventDefault();
    }
    function dragleave(event: DragEvent) {
        console.log("dragleave")
        // prevent default to allow drop
        event.preventDefault();
    }
</script>

<div>
  <div class="schedule-lane"
       on:drop={drop}
       on:dragover|preventDefault={dragover}
       on:dragenter|preventDefault={dragenter}
       on:dragleave|preventDefault={dragleave}>
    <div class="task"
         style="transform: {positionToStyle(start)};">
      <div class="left-handle" draggable="true"
      ></div>
      <div class="center-handle"
           draggable="true"
           on:dragstart|stopPropagation={dragstart}
           on:drag|preventDefault|stopPropagation={drag}
           on:dragend|stopPropagation={dragend}
           on:dragover|preventDefault
      >
        タスク名
      </div>
      <div class="right-handle" draggable="true"
      ></div>
    </div>
  </div>
</div>

<style>
    .schedule-lane {
        height: 40px;
        width: calc(40px * 10);
        /*flex-wrap: nowrap;*/
        /*display: flex;*/
        background-color: white;
    }
    .task {
        /*position: relative;*/
        position: absolute;
        flex-wrap: nowrap;
        /*display: flex;*/
        width: calc(40px * 4);
        height: 40px;
        background-color:gray;
    }
    .center-handle {
        position: absolute;
        background-color:gray;
        height: 100%;
        width: calc(40px * 4 - 10px * 2);
        left: 10px;
        /*flex-grow: 1;*/
        cursor: grab;
        z-index: 1;
    }
    .left-handle, .right-handle {
        position: absolute;
        width: 10px;
        height: 100%;
        cursor: grab;
        z-index: 2;
    }
    .left-handle {
        left: 0;
        border-radius: 4px 0 0 4px;
        background-color: red;
        /*height: 40px;*/
        /*min-width: 8px;*/
        /*width: 8px;*/
        /*cursor: grab;*/
        /*place-items: end;*/
    }
    .right-handle {
        right: 0;
        border-radius: 0 4px 4px 0;
        background-color: blue;
        /*height: 40px;*/
        /*min-width: 8px;*/
        /*width: 8px;*/
        /*cursor: grab;*/
    }
</style>