/* eslint-disable no-unused-vars */
/**
 * Pricecaster Service.
 *
 * Fetcher backend component.
 *
 * (c) 2021 Randlabs, Inc.
 */

import { PriceTicker } from './PriceTicker'
import { StatusCode } from './statusCodes'

export type PublishInfo = {
    status: StatusCode,
    reason?: '',
    msgb64?: '',
    block?: BigInt
    txid?: string
}

export interface IPublisher {
    start(): void
    stop(): void
    publish(tick: PriceTicker): Promise<PublishInfo>
}
