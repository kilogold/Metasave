using SubstrateNetApi;
using System;
using System.Collections;
using System.Collections.Generic;
using System.Numerics;
using System.Threading.Tasks;
using Schnorrkel.Keys;
using SubstrateNetApi.Model.Calls;
using SubstrateNetApi.Model.Types;
using SubstrateNetApi.Model.Types.Custom;
using UnityEngine;
using UnityEngine.UI;

public class MetaDataCrawler : MonoBehaviour
{
    private const string WEBSOCKETURL = "ws://127.0.0.1:9944";

    private SubstrateClient _client;

    public Image ImgConnect;

    public Text TxtConnect;

    public Text TxtButton;

    public Text TxtMetaData;

    public Text Text1, Text2, Text3;

    public InputField input;

    private Task _awaitableTask;

    // Start is called before the first frame update
    void Start()
    {
        
    }

    // Update is called once per frame
    void Update()
    {
        if (_awaitableTask != null && _awaitableTask.IsCompleted)
        {
            if (_client != null && _client.IsConnected)
            {
                ImgConnect.color = Color.green;
                TxtConnect.text = "On";
                TxtButton.text = "Disconnect";
                TxtMetaData.text = _client.MetaData.Serialize();
            } 
            else
            {
                ImgConnect.color = Color.red;
                TxtConnect.text = "Off";
                TxtButton.text = "Connect";
            }
            
            _awaitableTask = null;
        }
    }

    private async Task ConnectAsync()
    {
        _client = new SubstrateClient(new Uri(input.text));
        await _client.ConnectAsync();

        Text1.text = await _client.System.NameAsync();

        Text2.text = await _client.System.VersionAsync();

        Text3.text = await _client.System.ChainAsync();

        {
            // [Plain] Value: T::AccountId (from metaData)
            var reqResult = await _client.GetStorageAsync("TemplateModule", "Score");
            Debug.Log($"RESPONSE: '{reqResult}' [{reqResult.GetType().Name}]");
        }
        
        {
            /*
             * Secret Key URI `Kelvin` is account:
             * Secret seed:       0x227c672ec76923d1dcb3a670383c758a5dc46c952073559eda4a149cb0fdea1a
             * Public key (hex):  0xa6a25868698090a46184a216786cdc02e8c87aafa4502f8a4a504a1032c8421f
             * Account ID:        0xa6a25868698090a46184a216786cdc02e8c87aafa4502f8a4a504a1032c8421f
             * Public key (SS58): 5FqC2r8MawBvLb2V64rh2xa3VuXNY5K7v2Aq7XbtKq69CUq1
             * SS58 Address:      5FqC2r8MawBvLb2V64rh2xa3VuXNY5K7v2Aq7XbtKq69CUq1
             */
            MiniSecret MiniSecretKelvin = new MiniSecret(Utils.HexToByteArray("0x227c672ec76923d1dcb3a670383c758a5dc46c952073559eda4a149cb0fdea1a"), ExpandMode.Ed25519);
            Account Kelvin = Account.Build(KeyType.Sr25519, MiniSecretKelvin.ExpandToSecret().ToBytes(), MiniSecretKelvin.GetPair().Public.Key);
            
            // Secret Key URI `//Alice` is account:
            // Secret seed:      0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a
            // Public key(hex):  0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
            // Account ID:       0xd43593c715fdd31c61141abd04a99fd6822c8558854ccde39a5684e7a56da27d
            // SS58 Address:     5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
            MiniSecret MiniSecretAlice = new MiniSecret(Utils.HexToByteArray("0xe5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a"), ExpandMode.Ed25519);
            Account Alice = Account.Build(KeyType.Sr25519, MiniSecretAlice.ExpandToSecret().ToBytes(), MiniSecretAlice.GetPair().Public.Key);
            
            var incrementScore = new GenericExtrinsicCall("TemplateModule", "increment_score");
            var reqResult = await _client.Author.SubmitExtrinsicAsync(incrementScore, Kelvin, 0, 64);

            Debug.Log(reqResult.ToString());
        }
        
        
        // var publicKey = Utils.GetPublicKeyFrom("5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY");
        // {
        //     // [Map] Key: T::AccountId, Hasher: Blake2_128Concat, Value: AccountInfo<T::Index, T::AccountData> (from metaData)
        //     var reqResult = await _client.GetStorageAsync("System", "Account", new [] {Utils.Bytes2HexString(publicKey)});
        //     Debug.Log($"RESPONSE: '{reqResult}' [{reqResult.GetType().Name}]");
        // }
    }

    private async Task CloseAsync()
    {
        _client = new SubstrateClient(new Uri(WEBSOCKETURL));
        await _client.CloseAsync();
    }

    public void GetMetaDataClicked()
    {
        if (_client != null && _client.IsConnected)
        {
            _awaitableTask = CloseAsync();
            TxtMetaData.text = "";
            Text1.text = "";
            Text2.text = "";
            Text3.text = "";
        }
        else
        {
            _awaitableTask = ConnectAsync();

        }

    }
}
