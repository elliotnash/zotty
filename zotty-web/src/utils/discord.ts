const CDN_URL = "https://cdn.discordapp.com/";
export function get_avatar_url(user_id: string, avatar: string, size = 256){
  let url = new URL(CDN_URL);
  url.pathname = `avatars/${user_id}/${avatar}.webp`;
  url.search = new URLSearchParams({
    size: size.toString()
  }).toString();
  return url.toString();
}