<template>
  <div style="text-align: center;">
    <div v-if="signedIn" @click="logOut">Login out</div>
    <div v-if="!signedIn" @click="requestSignIn">Login in</div>

    <div>{{proposalsLen}}</div>
    <div>{{proposals}}</div>
    <div style="position:relative; display: inline-block; ">
      <vue-audio-mixer
        :config="config"
        size="medium"
        theme="dark"
        :showPan="true"
        :showTotalTime="true"
        @input="mixerInput"
        @loaded="mixerLoaded"
      />
    </div>
  </div>
</template>

<script>
import VueAudioMixer from "vue-audio-mixer";
import "vue-audio-mixer/dist/vue-audio-mixer.min.css";
import * as nearAPI from "near-api-js";
const ContractName = "dev-1629442049275-78217956173064";
const GAS = "200000000000000"

export const {
	utils: {
		format: {
			formatNearAmount, parseNearAmount
		}
	}
} = nearAPI;
export default {
  name: "App",
  components: {
    VueAudioMixer,
  },
  data: function() {
    return {
      is_loaded: false,
      newconfig: {},
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
      _walletConnection:null,
      proposals:null,
      proposalsLen:0,
      clearInternave:0,
      isInit:true
    };
  },
  created() {
    this._initNear().then(async () => {
      //debugger
      /* this.setState({
        connected: true,
        signedIn: !!this._accountId,
        accountId: this._accountId,
      }); */
      //debugger
      this.getProposals()
      //debugger
    });
  },
  methods: {
    async getProposals(){
      this.proposals = await this._contract.get_proposals({
        from_index: 0,
        limit: 100,
      });
      this.proposalsLen = await this._contract.get_num_proposals();
    },
    async mixerInput() {
      //debugger;
      if(this.isInit){
        this.isInit = false
        return;
      }
      let data = {
          proposal: {
            target: "paul",
            description: "test",
            kind: {
              type: "Payout",
              amount: "1000000000000000000000000",
            },
          },
      };
      clearTimeout(this.clearInternave)
      this.clearInternave = setTimeout(async ()=>{
        let res = await this._contract.add_proposal(data,GAS,parseNearAmount('1'))
        this.getProposals()
      },1000)
      //debugger
    },
    mixerLoaded() {
      //debugger;
    },
    async requestSignIn() {
      const appTitle = "NEAR NFT";
      await this._walletConnection.requestSignIn(ContractName, appTitle);
    },

    async logOut() {
      this._walletConnection.signOut();
      this._accountId = null;
      this.signedIn = !!this._accountId
      this.accountId = this._accountId
    },
    async _initNear() {
      const nearConfig = {
        networkId: "default",
        nodeUrl: "https://rpc.testnet.near.org",
        contractName: ContractName,
        walletUrl: "https://wallet.testnet.near.org",
      };
      const keyStore = new nearAPI.keyStores.BrowserLocalStorageKeyStore();
      const near = await nearAPI.connect(
        Object.assign({ deps: { keyStore } }, nearConfig)
      );
      this._keyStore = keyStore;
      this._nearConfig = nearConfig;
      this._near = near;

      this._walletConnection = new nearAPI.WalletConnection(near, ContractName);
      this._accountId = this._walletConnection.getAccountId();

      this._account = this._walletConnection.account();
      this.signedIn = !!this._accountId
      //debugger
      this._contract = new nearAPI.Contract(this._account, ContractName, {
        viewMethods: ["get_proposals", "get_num_proposals"],
        changeMethods: ["add_proposal"],
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
</style>
