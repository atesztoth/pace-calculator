export const createPromiseSleep = (ms: number): Promise<void> => new Promise((resolve) => setTimeout(resolve, ms))
