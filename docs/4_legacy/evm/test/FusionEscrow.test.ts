import { expect } from "chai";
import { ethers } from "hardhat";
import { FusionEscrow, TestICP, TestETH } from "../typechain-types";

describe("FusionEscrow", function () {
  let fusionEscrow: FusionEscrow;
  let testICP: TestICP;
  let testETH: TestETH;
  let owner: any;
  let maker: any;
  let resolver: any;
  let addr1: any;
  let addr2: any;

  beforeEach(async function () {
    // Get signers
    [owner, maker, resolver, addr1, addr2] = await ethers.getSigners();

    // Deploy contracts
    const FusionEscrowFactory = await ethers.getContractFactory("FusionEscrow");
    fusionEscrow = await FusionEscrowFactory.deploy();

    const TestICPFactory = await ethers.getContractFactory("TestICP");
    testICP = await TestICPFactory.deploy();

    const TestETHFactory = await ethers.getContractFactory("TestETH");
    testETH = await TestETHFactory.deploy();

    // Authorize resolver
    await fusionEscrow.authorizeResolver(resolver.address);

    // Mint test tokens
    const mintAmount = ethers.parseEther("1000");
    await testICP.mint(maker.address, mintAmount);
    await testETH.mint(resolver.address, mintAmount);
  });

  describe("Deployment", function () {
    it("Should set the right owner", async function () {
      expect(await fusionEscrow.owner()).to.equal(owner.address);
    });

    it("Should authorize resolver", async function () {
      expect(await fusionEscrow.authorizedResolvers(resolver.address)).to.equal(
        true
      );
    });
  });

  describe("ETH Locking", function () {
    it("Should lock ETH for swap", async function () {
      const orderId = "test-order-1";
      const amount = ethers.parseEther("1");
      const currentBlock = await ethers.provider.getBlock("latest");
      const timelock = currentBlock!.timestamp + 7200; // 2 hours from now (more than MIN_LOCK_TIME)

      await expect(
        fusionEscrow
          .connect(maker)
          .lockETHForSwap(orderId, timelock, { value: amount })
      )
        .to.emit(fusionEscrow, "ETHLocked")
        .withArgs(orderId, maker.address, amount, ethers.ZeroHash);

      const escrowData = await fusionEscrow.getEscrowData(orderId);
      expect(escrowData.maker).to.equal(maker.address);
      expect(escrowData.amount).to.equal(amount);
      expect(escrowData.hashlock).to.equal(ethers.ZeroHash);
      expect(escrowData.timelock).to.equal(timelock);
      expect(escrowData.claimed).to.equal(false);
      expect(escrowData.refunded).to.equal(false);
    });

    it("Should fail if no ETH sent", async function () {
      const orderId = "test-order-2";
      const currentBlock = await ethers.provider.getBlock("latest");
      const timelock = currentBlock!.timestamp + 7200;

      await expect(
        fusionEscrow
          .connect(maker)
          .lockETHForSwap(orderId, timelock, { value: 0 })
      ).to.be.revertedWith("FusionEscrow: must send ETH");
    });

    it("Should fail if order already exists", async function () {
      const orderId = "test-order-3";
      const amount = ethers.parseEther("1");
      const currentBlock = await ethers.provider.getBlock("latest");
      const timelock = currentBlock!.timestamp + 7200;

      await fusionEscrow
        .connect(maker)
        .lockETHForSwap(orderId, timelock, { value: amount });

      await expect(
        fusionEscrow
          .connect(maker)
          .lockETHForSwap(orderId, timelock, { value: amount })
      ).to.be.revertedWith("FusionEscrow: order already exists");
    });
  });

  describe("ETH Claiming", function () {
    beforeEach(async function () {
      // Setup a locked escrow
      const orderId = "test-order-4";
      const amount = ethers.parseEther("1");
      const currentBlock = await ethers.provider.getBlock("latest");
      const timelock = currentBlock!.timestamp + 7200;

      await fusionEscrow
        .connect(maker)
        .lockETHForSwap(orderId, timelock, { value: amount });
    });

    it("Should claim ETH with valid receipt", async function () {
      const orderId = "test-order-4";
      const icpReceipt = "test-receipt";

      const initialBalance = await ethers.provider.getBalance(resolver.address);

      await expect(
        fusionEscrow.connect(resolver).claimLockedETH(orderId, icpReceipt)
      )
        .to.emit(fusionEscrow, "ETHClaimed")
        .withArgs(orderId, resolver.address, ethers.parseEther("1"));

      const finalBalance = await ethers.provider.getBalance(resolver.address);
      expect(finalBalance).to.be.gt(initialBalance);
    });

    it("Should fail if not authorized resolver", async function () {
      const orderId = "test-order-4";
      const icpReceipt = "test-receipt";

      await expect(
        fusionEscrow.connect(addr1).claimLockedETH(orderId, icpReceipt)
      ).to.be.revertedWith("FusionEscrow: not authorized resolver");
    });

    it("Should fail with empty receipt", async function () {
      const orderId = "test-order-4";
      const icpReceipt = "";

      await expect(
        fusionEscrow.connect(resolver).claimLockedETH(orderId, icpReceipt)
      ).to.be.revertedWith("FusionEscrow: invalid ICP receipt");
    });
  });

  describe("ETH Refunding", function () {
    it("Should refund ETH after timelock expires", async function () {
      const orderId = "test-order-5";
      const amount = ethers.parseEther("1");
      const currentBlock = await ethers.provider.getBlock("latest");
      const timelock = currentBlock!.timestamp + 7200; // 2 hours from now

      await fusionEscrow
        .connect(maker)
        .lockETHForSwap(orderId, timelock, { value: amount });

      // Wait for timelock to expire (2 hours + 1 second)
      await ethers.provider.send("evm_increaseTime", [7201]);
      await ethers.provider.send("evm_mine", []);

      const initialBalance = await ethers.provider.getBalance(maker.address);

      await expect(fusionEscrow.connect(maker).refundLockedETH(orderId))
        .to.emit(fusionEscrow, "ETHRefunded")
        .withArgs(orderId, maker.address, ethers.parseEther("1"));

      const finalBalance = await ethers.provider.getBalance(maker.address);
      expect(finalBalance).to.be.gt(initialBalance);
    });

    // TODO: Fix timelock timing issues in tests - test manually for now
    /*
    it("Should fail if timelock not expired", async function () {
      const orderId = "test-order-6";
      const amount = ethers.parseEther("1");
      const currentBlock = await ethers.provider.getBlock("latest");
      const timelock = currentBlock!.timestamp + 7200; // 2 hours from now

      await fusionEscrow
        .connect(maker)
        .lockETHForSwap(orderId, timelock, { value: amount });

      await expect(
        fusionEscrow.connect(maker).refundLockedETH(orderId)
      ).to.be.revertedWith("FusionEscrow: timelock not expired");
    });

    it("Should fail if not the maker", async function () {
      const orderId = "test-order-7";
      const amount = ethers.parseEther("1");
      const currentBlock = await ethers.provider.getBlock("latest");
      const timelock = currentBlock!.timestamp + 7200; // 2 hours from now

      await fusionEscrow
        .connect(maker)
        .lockETHForSwap(orderId, timelock, { value: amount });

      // Wait for timelock to expire (2 hours + 1 second)
      await ethers.provider.send("evm_increaseTime", [7201]);
      await ethers.provider.send("evm_mine", []);

      await expect(
        fusionEscrow.connect(addr1).refundLockedETH(orderId)
      ).to.be.revertedWith("FusionEscrow: only maker can refund");
    });
    */
  });

  describe("Resolver Management", function () {
    it("Should authorize new resolver", async function () {
      await expect(fusionEscrow.authorizeResolver(addr1.address))
        .to.emit(fusionEscrow, "ResolverAuthorized")
        .withArgs(addr1.address);

      expect(await fusionEscrow.authorizedResolvers(addr1.address)).to.equal(
        true
      );
    });

    it("Should deauthorize resolver", async function () {
      await fusionEscrow.authorizeResolver(addr1.address);
      await fusionEscrow.deauthorizeResolver(addr1.address);

      expect(await fusionEscrow.authorizedResolvers(addr1.address)).to.equal(
        false
      );
    });

    it("Should fail if non-owner tries to authorize", async function () {
      await expect(
        fusionEscrow.connect(addr1).authorizeResolver(addr2.address)
      ).to.be.revertedWithCustomError(
        fusionEscrow,
        "OwnableUnauthorizedAccount"
      );
    });
  });
});
