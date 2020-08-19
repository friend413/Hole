import React, {createContext, FunctionComponent, useContext} from "react"
import ClientContext from "../providers/ClientContext";
import solanaWeb3, {Connection, PublicKey} from "@solana/web3.js";
import {SolanaBridge} from "../utils/bridge";
import {SOLANA_BRIDGE_PROGRAM, TOKEN_PROGRAM} from "../config";

export const BridgeContext = createContext<SolanaBridge>(new SolanaBridge(new Connection(""),SOLANA_BRIDGE_PROGRAM, TOKEN_PROGRAM));

export const BridgeProvider: FunctionComponent = ({children}) => {
    let c = useContext<solanaWeb3.Connection>(ClientContext);

    let bridge = new SolanaBridge(c, SOLANA_BRIDGE_PROGRAM,TOKEN_PROGRAM)

    return (
        <BridgeContext.Provider value={bridge}>
            {children}
        </BridgeContext.Provider>
    )
}
