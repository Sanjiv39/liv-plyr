export type ContentType =
  | "application/json"
  | "application/xml"
  | "application/x-www-form-urlencoded"
  | "multipart/form-data"
  | "text/plain"
  | "text/html"
  | "text/css"
  | "text/javascript"
  | "image/jpeg"
  | "image/png"
  | "image/gif"
  | "audio/mpeg"
  | "video/mp4"
  | "application/pdf"
  | "application/octet-stream"
  // HLS (HTTP Live Streaming) related types
  | "application/vnd.apple.mpegurl" // .m3u8 playlist
  | "audio/mpegurl" // Older .m3u8 for audio
  | "video/mpegurl" // Older .m3u8 for video
  | "application/x-mpegURL" // Common for .m3u8
  | "video/mp2t" // .ts (MPEG-2 Transport Stream) segments
  // DASH (Dynamic Adaptive Streaming over HTTP) related types
  | "application/dash+xml"; // .mpd (Media Presentation Description)
