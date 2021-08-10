import type { DiscordUser } from "./request";

const CDN_URL = "https://cdn.discordapp.com/";
export function getAvatarUrl(user: DiscordUser, size = 256): string {
  const url = new URL(CDN_URL);
  if (user.avatar)
    url.pathname = `avatars/${user.id}/${user.avatar}.webp`;
  else
    url.pathname = `embed/avatars/${parseInt(user.discriminator) % 5}.webp`;
  url.search = new URLSearchParams({
    size: size.toString()
  }).toString();
  return url.toString();
}