import {invoke} from "@tauri-apps/api/core";
import {EarnedValueManagementInfo} from "$components/EarnedValueManagementInfo";
import type {PlannedValueChanges} from "$components/PlannedValueChanges";
import type {EvmHistory} from "$components/EvmHistory";


/** @type {import('./$types').PageLoad} */
export async function load({params}) {
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
            evm_info_p,
            planned_value_changes,
            evm_histories,
        ]);
    return {
        evm_info: ret[0],
        planned_value_changes: ret[1],
        evm_histories: ret[2],
    }
}
