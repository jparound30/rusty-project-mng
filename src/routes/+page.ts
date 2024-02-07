import { redirect } from '@sveltejs/kit';

/** @type {import('./$types').PageLoad} */
export function load({ params }) {
    // TODO キャッシュなどから認証状態を復元したりする
    // TODO とりあえず必ず /login に飛ばす
    // redirect(307, '/login')
    redirect(307, '/home')
}