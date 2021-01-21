export const expiries: {
  [name: string]: number;
} = {
  "Delete after read": -1,
  "1 minute": 60,
  "10 minutes": 600,
  "1 hour": 3600,
  "12 hours": 12 * 3600,
  "1 day": 24 * 3600,
  "2 days": 2 * 24 * 3600,
  "1 week": 7 * 24 * 3600
};
