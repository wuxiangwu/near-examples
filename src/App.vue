<template>
  <div style="text-align: center;">
    <!-- <div>{{proposalsLen}}</div>
    <div>{{proposals}}</div> -->
    <div style="position:relative; display: inline-block; ">
      <vue-audio-mixer
        :config="config"
        size="medium"
        theme="dark"
        :showPan="true"
        :showTotalTime="true"
        @input="mixerInput"
        @mint="mixerMint"
        @save="mixerSave"
        @loaded="mixerLoaded"
      />
    </div>
    <div style="margin:16px 0;">
      <button class="btn" v-if="signedIn" @click="logOut">退出登录</button>
      <button class="btn" v-if="!signedIn" @click="requestSignIn">登录</button>
    </div>
    <div style="margin:16px 0;">receiver：<input :value="receiver"/></div>
    <div>
      <a
        target="_blank"
        href="https://explorer.testnet.near.org/accounts/rhythm4nft.testnet"
        >去浏览器查看交易信息</a
      >
    </div>
  </div>
</template>

<script>
//import VueAudioMixer from "vue-audio-mixer";
//import "vue-audio-mixer/dist/vue-audio-mixer.min.css";
import VueAudioMixer from "./components/vue-audio-mixer/dist/vue-audio-mixer.esm.js";
import "./components/vue-audio-mixer/dist/vue-audio-mixer.esm.css";
import * as nearAPI from "near-api-js";
import getConfig from "./config";

export const {
  GAS,
  networkId,
  nodeUrl,
  walletUrl,
  nameSuffix,
  contractName,
  contractMethods,
} = getConfig();

export const {
  utils: {
    format: { formatNearAmount, parseNearAmount },
  },
} = nearAPI;
export default {
  name: "App",
  components: {
    VueAudioMixer,
  },
  data: function() {
    return {
      is_loaded: false,
      receiver: "paultest.testnet",
      newconfig: {},
      tokenId: "",
      config: {
        tracks: [
          {
            title: "Bass",
            url:
              "https://api.soundcloud.com/tracks/841840237/stream?client_id=ae1dadcc70f054f451de8c6358bcf396",
            pan: -30,
            gain: 0.5,
            muted: false,
            hidden: false,
          },
          {
            title: "Flutes",
            url:
              "https://api.soundcloud.com/tracks/841840234/stream?client_id=ae1dadcc70f054f451de8c6358bcf396",
            pan: 81,
            gain: 1.08,
            muted: false,
            hidden: false,
          },
          {
            title: "Perc",
            url:
              "https://api.soundcloud.com/tracks/841840222/stream?client_id=ae1dadcc70f054f451de8c6358bcf396",
            pan: -49,
            gain: 0.85,
            muted: false,
            hidden: false,
          },
          {
            title: "Piano",
            url:
              "https://api.soundcloud.com/tracks/841840216/stream?client_id=ae1dadcc70f054f451de8c6358bcf396",
            pan: -60,
            gain: 0.6,
            muted: false,
            hidden: false,
          },
          {
            title: "Strings",
            url:
              "https://api.soundcloud.com/tracks/841840174/stream?client_id=ae1dadcc70f054f451de8c6358bcf396",
            pan: -49,
            gain: 0.85,
            muted: false,
            hidden: false,
          },
          {
            title: "Bass",
            url:
              "https://api.soundcloud.com/tracks/841840237/stream?client_id=ae1dadcc70f054f451de8c6358bcf396",
            pan: -30,
            gain: 0.5,
            muted: false,
            hidden: false,
          },
        ],
        master: {
          pan: 0,
          gain: 1,
          muted: false,
        },
      },
      signedIn: false,
      accountId: "",
      _walletConnection: null,
      proposals: null,
      proposalsLen: 0,
      clearInternave: 0,
      isInit: true,
    };
  },
  created() {
    this._initNear().then(async () => {});
  },
  methods: {
    async mixerSave(res) {
      if (!this.signedIn) {
        alert("请先登录！");
        return;
      }
      if (!this.tokenId) {
        alert("请先mint！");
        return;
      }
      let data = {
        token_id: this.tokenId,
        receiver_id: this.receiver,
        memo: "111transfer ownership",
      };
      let ret = await this._contract.nft_transfer(
        data,
        GAS,
        parseNearAmount("0.000000000000000000000001")
      ); //,GAS,parseNearAmount('1'))
      alert("transfer成功");
      /* window.location.href =
        "https://explorer.testnet.near.org/accounts/rhythm4nft.testnet"; */
    },
    async mixerMint(res) {
      if (!this.signedIn) {
        alert("请先登录！");
        return;
      }
      this.tokenId = "tid" + new Date().getTime();
      let data = {
        token_id: this.tokenId,
        token_owner_id: "rhythm4nft.testnet", // this._accountId,
        token_metadata: {
          title: "Olympus Mons",
          config: JSON.stringify(res),
          description: "Tallest mountain in charted solar system",
          copies: 1,
        },
      };
      let ret = await this._contract.nft_mint(data, GAS, parseNearAmount("1")); //,GAS,parseNearAmount('1'))
      //let ret = await this._contract.nft_metadata()
      alert("mint成功");
    },
    async mixerInput(res) {
      /* if (this.isInit) {
        this.isInit = false;
        return;
      }
      let data = {
        claim: JSON.stringify(res),
      };
      clearTimeout(this.clearInternave);
      this.clearInternave = setTimeout(async () => {
        let res = await this._contract.insert_claim(data); //,GAS,parseNearAmount('1'))
        console.log("res:", res);
      }, 1000); */
    },
    mixerLoaded() {},
    async requestSignIn() {
      const appTitle = "NEAR NFT";
      await this._walletConnection.requestSignIn(contractName, appTitle);
    },

    async logOut() {
      this._walletConnection.signOut();
      this._accountId = null;
      this.signedIn = !!this._accountId;
      this.accountId = this._accountId;
    },
    async _initNear() {
      const nearConfig = {
        networkId,
        nodeUrl,
        contractName: contractName,
        walletUrl,
      };
      const keyStore = new nearAPI.keyStores.BrowserLocalStorageKeyStore();
      const near = await nearAPI.connect(
        Object.assign({ deps: { keyStore } }, nearConfig)
      );
      this._keyStore = keyStore;
      this._nearConfig = nearConfig;
      this._near = near;

      this._walletConnection = new nearAPI.WalletConnection(near, contractName);
      this._accountId = this._walletConnection.getAccountId();

      this._account = this._walletConnection.account();
      this.signedIn = !!this._accountId;
      //debugger
      this._contract = new nearAPI.Contract(this._account, contractName, {
        ...contractMethods,
      });
    },
  },
};
</script>

<style>
#app {
  font-family: "Avenir", Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
.btn {
  border-radius: 12px;
  background-color: blue;
  color: #fff;
  padding: 8px 12px;
}
</style>
