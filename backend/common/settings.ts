export interface IAppSettings extends Record<string, unknown> {
    algo: {
        token: string,
        api: string,
        port: string,
    },
    pyth?: {
        solanaClusterName?: string
    },
    params: {
        verbose?: boolean,
        symbol: string,
        bufferSize: number,
        publishIntervalSecs: number,
        priceKeeperAppId: BigInt,
        validator: string,
        mnemo?: string
    }
}
