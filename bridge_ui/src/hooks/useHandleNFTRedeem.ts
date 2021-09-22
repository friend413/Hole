import {
  CHAIN_ID_ETH,
  CHAIN_ID_SOLANA,
  getClaimAddressSolana,
  postVaaSolana,
  parseNFTPayload,
  hexToUint8Array,
} from "@certusone/wormhole-sdk";
import {
  createMetaOnSolana,
  getForeignAssetSol,
  isNFTVAASolanaNative,
  redeemOnEth,
  redeemOnSolana,
} from "@certusone/wormhole-sdk/lib/nft_bridge";
import { arrayify } from "@ethersproject/bytes";
import { WalletContextState } from "@solana/wallet-adapter-react";
import { Connection } from "@solana/web3.js";
import { Signer } from "ethers";
import { useSnackbar } from "notistack";
import { useCallback, useMemo } from "react";
import { useDispatch, useSelector } from "react-redux";
import { useEthereumProvider } from "../contexts/EthereumProviderContext";
import { useSolanaWallet } from "../contexts/SolanaWalletContext";
import { setIsRedeeming, setRedeemTx } from "../store/nftSlice";
import { selectNFTIsRedeeming, selectNFTTargetChain } from "../store/selectors";
import {
  ETH_NFT_BRIDGE_ADDRESS,
  SOLANA_HOST,
  SOL_BRIDGE_ADDRESS,
  SOL_NFT_BRIDGE_ADDRESS,
} from "../utils/consts";
import { getMetadataAddress } from "../utils/metaplex";
import parseError from "../utils/parseError";
import { signSendAndConfirm } from "../utils/solana";
import useNFTSignedVAA from "./useNFTSignedVAA";

async function eth(
  dispatch: any,
  enqueueSnackbar: any,
  signer: Signer,
  signedVAA: Uint8Array
) {
  dispatch(setIsRedeeming(true));
  try {
    const receipt = await redeemOnEth(
      ETH_NFT_BRIDGE_ADDRESS,
      signer,
      signedVAA
    );
    dispatch(
      setRedeemTx({ id: receipt.transactionHash, block: receipt.blockNumber })
    );
    enqueueSnackbar("Transaction confirmed", { variant: "success" });
  } catch (e) {
    enqueueSnackbar(parseError(e), { variant: "error" });
    dispatch(setIsRedeeming(false));
  }
}

async function solana(
  dispatch: any,
  enqueueSnackbar: any,
  wallet: WalletContextState,
  payerAddress: string, //TODO: we may not need this since we have wallet
  signedVAA: Uint8Array
) {
  dispatch(setIsRedeeming(true));
  try {
    if (!wallet.signTransaction) {
      throw new Error("wallet.signTransaction is undefined");
    }
    const connection = new Connection(SOLANA_HOST, "confirmed");
    const claimAddress = await getClaimAddressSolana(
      SOL_NFT_BRIDGE_ADDRESS,
      signedVAA
    );
    const claimInfo = await connection.getAccountInfo(claimAddress);
    let txid;
    if (!claimInfo) {
      await postVaaSolana(
        connection,
        wallet.signTransaction,
        SOL_BRIDGE_ADDRESS,
        payerAddress,
        Buffer.from(signedVAA)
      );
      // TODO: how do we retry in between these steps
      const transaction = await redeemOnSolana(
        connection,
        SOL_BRIDGE_ADDRESS,
        SOL_NFT_BRIDGE_ADDRESS,
        payerAddress,
        signedVAA
      );
      txid = await signSendAndConfirm(wallet, connection, transaction);
      // TODO: didn't want to make an info call we didn't need, can we get the block without it by modifying the above call?
    }
    const isNative = await isNFTVAASolanaNative(signedVAA);
    if (!isNative) {
      const { parse_vaa } = await import(
        "@certusone/wormhole-sdk/lib/solana/core/bridge"
      );
      const parsedVAA = parse_vaa(signedVAA);
      const { originChain, originAddress, tokenId } = parseNFTPayload(
        Buffer.from(new Uint8Array(parsedVAA.payload))
      );
      const mintAddress = await getForeignAssetSol(
        SOL_NFT_BRIDGE_ADDRESS,
        originChain,
        hexToUint8Array(originAddress),
        arrayify(tokenId)
      );
      const [metadataAddress] = await getMetadataAddress(mintAddress);
      const metadata = await connection.getAccountInfo(metadataAddress);
      if (!metadata) {
        const transaction = await createMetaOnSolana(
          connection,
          SOL_BRIDGE_ADDRESS,
          SOL_NFT_BRIDGE_ADDRESS,
          payerAddress,
          signedVAA
        );
        txid = await signSendAndConfirm(wallet, connection, transaction);
      }
    }
    dispatch(setRedeemTx({ id: txid || "", block: 1 }));
    enqueueSnackbar("Transaction confirmed", { variant: "success" });
  } catch (e) {
    enqueueSnackbar(parseError(e), { variant: "error" });
    dispatch(setIsRedeeming(false));
  }
}

export function useHandleNFTRedeem() {
  const dispatch = useDispatch();
  const { enqueueSnackbar } = useSnackbar();
  const targetChain = useSelector(selectNFTTargetChain);
  const solanaWallet = useSolanaWallet();
  const solPK = solanaWallet?.publicKey;
  const { signer } = useEthereumProvider();
  const signedVAA = useNFTSignedVAA();
  const isRedeeming = useSelector(selectNFTIsRedeeming);
  const handleRedeemClick = useCallback(() => {
    if (targetChain === CHAIN_ID_ETH && !!signer && signedVAA) {
      eth(dispatch, enqueueSnackbar, signer, signedVAA);
    } else if (
      targetChain === CHAIN_ID_SOLANA &&
      !!solanaWallet &&
      !!solPK &&
      signedVAA
    ) {
      solana(
        dispatch,
        enqueueSnackbar,
        solanaWallet,
        solPK.toString(),
        signedVAA
      );
    } else {
      // enqueueSnackbar("Redeeming on this chain is not yet supported", {
      //   variant: "error",
      // });
    }
  }, [
    dispatch,
    enqueueSnackbar,
    targetChain,
    signer,
    signedVAA,
    solanaWallet,
    solPK,
  ]);
  return useMemo(
    () => ({
      handleClick: handleRedeemClick,
      disabled: !!isRedeeming,
      showLoader: !!isRedeeming,
    }),
    [handleRedeemClick, isRedeeming]
  );
}
