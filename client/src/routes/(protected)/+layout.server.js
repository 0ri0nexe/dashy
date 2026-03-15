import { redirect } from '@sveltejs/kit';

export const load = async ({ cookies }) => {

  const session = cookies.get("session_id");

  if (!session) {
    throw redirect(302, "/login");
  }

};