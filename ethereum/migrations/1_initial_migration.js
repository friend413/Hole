const WrappedAsset = artifacts.require("WrappedAsset");
const Wormhole = artifacts.require("Wormhole");
const ERC20 = artifacts.require("ERC20PresetMinterPauser");

module.exports = async function (deployer) {
    await deployer.deploy(WrappedAsset);
    await deployer.deploy(Wormhole, {
        keys: ["0xbeFA429d57cD18b7F8A4d91A2da9AB4AF05d0FBe"],
        expiration_time: 0
    }, WrappedAsset.address, 1000);
    await deployer.deploy(ERC20, "Test Token","TKN");
};
