import {invoke} from "@tauri-apps/api/tauri";

class TaskFull {
    task_id: number = 0;
    title: string = "";
    description: string | null = null;
    assignee_resource_id: number | null = null;
    assignee_resource_name: string | null = null;
    parent_task_id: number | null = null;
    parent_task_title: string | null = null;
    start_date: string | null = null;
    due_date: string | null = null;
    estimated_time: number | null = null;
    actual_time: number | null = null;
    planed_value: number | null = null;
    task_status_id: number = 0;
    task_status_name: String = "";
    progress_rate: number = 0;
}

/** @type {import('./$types').PageLoad} */
export async function load({params}) {
    let task_list =
        await invoke<TaskFull[]>("task_all_full", {})
            .then(value => {
                console.log("タスク（idとタイトルのみ）一覧取得成功")
                return value
            }).catch(reason => {
                console.error("タスク（idとタイトルのみ）一覧取得失敗", reason)
                return [] as TaskFull[];
            })

    return {
        task_list,
    }
}
