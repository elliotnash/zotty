import { DiscordUser } from "../types";

const CDN_URL = "https://cdn.discordapp.com/";
export function getAvatarUrl(user: DiscordUser | undefined, size = 256){
  if (!user){
    return;
  }
  let url = new URL(CDN_URL);
  if (user.avatar)
    url.pathname = `avatars/${user.id}/${user.avatar}.webp`;
  else
    url.pathname = `embed/avatars/${parseInt(user.discriminator) % 5}.webp`
  url.search = new URLSearchParams({
    size: size.toString()
  }).toString();
  return url.toString();
}