import {invoke} from "@tauri-apps/api/tauri";

/** @type {import('./$types').PageLoad} */
export async function load({params}) {
    let task_status_list =
        await invoke<{ task_status_id: number, title: string, view_order: number }[]>("task_status_list", {})
            .then(value => {
                console.log("ステータス一覧取得成功")
                return value
            }).catch(reason => {
                console.error("ステータス一覧取得失敗", reason)
                return [] as { task_status_id: number, title: string, view_order: number }[];
            })
    return {
        task_status_list
    }
}
