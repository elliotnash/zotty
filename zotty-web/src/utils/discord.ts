import type { DiscordUser, PartialGuild } from "./request";

const CDN_URL = "https://cdn.discordapp.com/";
export function getAvatarUrl(user: DiscordUser, size = 256): string {
  const url = new URL(CDN_URL);
  if (user.avatar)
    url.pathname = `avatars/${user.id}/${user.avatar}.webp`;
  else
    url.pathname = `embed/avatars/${parseInt(user.discriminator) % 5}.png`;
  url.search = new URLSearchParams({
    size: size.toString()
  }).toString();
  return url.toString();
}

export function getGuildIconUrl(guild: PartialGuild, size = 256): string {
  const url = new URL(CDN_URL);
  if (guild.icon)
    url.pathname = `icons/${guild.id}/${guild.icon}.webp`;
  else
    url.pathname = `embed/avatars/0.png`;
  url.search = new URLSearchParams({
    size: size.toString()
  }).toString();
  return url.toString();
}
