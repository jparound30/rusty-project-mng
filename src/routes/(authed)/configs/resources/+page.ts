import {invoke} from "@tauri-apps/api/core";
import type {Resource} from "components/Resource";


/** @type {import('../../../../../.svelte-kit/types/src/routes').PageLoad} */
export async function load({params}) {
    let resource_list_p =
        invoke<Resource[]>("resource_list", {})
            .then(value => {
                console.log("リソース一覧取得成功")
                return value
            }).catch(reason => {
            console.error("リソース一覧取得失敗", reason)
            return [] as Resource[];
        })

    const ret =
        await Promise.all([
            resource_list_p,
        ]);
    return {
        resource_list: ret[0],
    }
}
