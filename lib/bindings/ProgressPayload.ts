// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ProgressDetails } from "./ProgressDetails";
import type { ProgressTask } from "./ProgressTask";

export interface ProgressPayload {
  details: ProgressDetails;
  currentTask: ProgressTask;
  percentage: number;
  runningTotal: number;
}
