import {invoke} from "@tauri-apps/api/core";
import type {Resource} from "$components/Resource";
import type {TaskFull} from "$components/TaskFull";
import {EarnedValueManagementInfo} from "$components/EarnedValueManagementInfo";
import type {PlannedValueChanges} from "$components/PlannedValueChanges";
import type {EvmHistory} from "$components/EvmHistory";


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

    let resource_list_p =
        invoke<Resource[]>("resource_list", {})
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

    let planned_value_changes = invoke<PlannedValueChanges[]>("get_planned_value_changes", {})
        .then(value => {
            console.log("PV変化の一覧取得成功")
            return value
        }).catch(reason => {
            console.error("PV変化の一覧取得取得失敗", reason)
            return [] as PlannedValueChanges[];
        })

    let evm_histories = invoke<EvmHistory[]>("get_evm_histories", {})
        .then(value => {
            console.log("AC/EVの履歴取得成功")
            return value
        }).catch(reason => {
            console.error("AC/EVの履歴取得失敗", reason)
            return [] as EvmHistory[];
        })

    const ret =
        await Promise.all([
            task_list_p,
            resource_list_p,
            evm_info_p,
            planned_value_changes,
            evm_histories,
        ]);
    return {
        task_list: ret[0],
        resource_list: ret[1],
        evm_info: ret[2],
        planned_value_changes: ret[3],
        evm_histories: ret[4],
    }
}
