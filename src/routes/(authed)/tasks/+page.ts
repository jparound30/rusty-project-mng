import {invoke} from "@tauri-apps/api/core";
import type {Resource} from "$components/Resource";
import type {TaskFull} from "$components/TaskFull";
import {EarnedValueManagementInfo} from "components/EarnedValueManagementInfo";


/** @type {import('./$types').PageLoad} */
export async function load({params}) {
    let task_list_p =
        invoke<TaskFull[]>("task_all_full", {})
            .then(value => {
                console.log("タスク（idとタイトルのみ）一覧取得成功")
                return value
            }).catch(reason => {
                console.error("タスク（idとタイトルのみ）一覧取得失敗", reason)
                return [] as TaskFull[];
            })

    let resources_list_p =
        invoke<Resource[]>("resources_list", {})
            .then(value => {
                console.log("リソース一覧取得成功")
                return value
            }).catch(reason => {
                console.error("リソース一覧取得失敗", reason)
                return [] as Resource[];
            })

    let evm_info_p =
        invoke<EarnedValueManagementInfo>("get_current_evm_info", {})
        .then(value => {
            console.log("現時点のEVMの各情報の取得成功")
            return value
        }).catch(reason => {
            console.error("現時点のEVMの各情報の取得失敗", reason)
            return new EarnedValueManagementInfo();
        })

    const ret = await Promise.all([task_list_p, resources_list_p, evm_info_p]);
    return {
        task_list : ret[0],
        resources_list: ret[1],
        evm_info: ret[2],
    }
}
