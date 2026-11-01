// Constant request rate (open loop). One run = one target, one path, one rate.
// Env: BASE_URL, REQ_PATH, RATE (req/s), DURATION (e.g. "10s"), API_KEY (optional)
import http from "k6/http";
import { check } from "k6";
import { Options } from "k6/options";

const baseUrl = __ENV.BASE_URL;
const path = __ENV.REQ_PATH;
const rate = Number(__ENV.RATE);
const apiKey = __ENV.API_KEY;

export const options: Options = {
  scenarios: {
    load: {
      executor: "constant-arrival-rate",
      rate: rate,
      timeUnit: "1s",
      duration: __ENV.DURATION || "10s",
      preAllocatedVUs: 50,
      maxVUs: 1000,
    },
  },
  summaryTrendStats: ["med", "p(95)", "p(99)", "max"],
};

export default function () {
  const headers: Record<string, string> = {};
  if (apiKey) {
    headers["Authorization"] = `Bearer ${apiKey}`;
  }
  const response = http.get(`${baseUrl}/${path}`, { headers });
  check(response, { "status is 200": (r) => r.status === 200 });
}
