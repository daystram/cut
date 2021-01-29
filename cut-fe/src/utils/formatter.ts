export function formatUnit(size: number) {
  if (size > 1000 * 1000) {
    return (size / (1000 * 1000)).toFixed(2) + " MB";
  } else if (size > 1000) {
    return (size / 1000).toFixed(2) + " KB";
  }
  return size.toString() + " B";
}
