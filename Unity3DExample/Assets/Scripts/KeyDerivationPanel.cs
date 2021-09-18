//using sr25519_dotnet.lib;
using Chaos.NaCl;
using System;
using System.Collections;
using System.Collections.Generic;
using System.Text;
using UnityEngine;
using UnityEngine.UI;

public class KeyDerivationPanel : MonoBehaviour
{
    private System.Random _random;

    public Text Secret;

    public Text PublicKey;

    public Text Message;

    // Start is called before the first frame update
    void Start()
    {

        _random = new System.Random();
    }

    // Update is called once per frame
    void Update()
    {
        
    }

    public void GenerateClicked()
    {
        var seed = new byte[32];
        _random.NextBytes(seed);

        byte[] pubKey, priKey;
        Ed25519.KeyPairFromSeed(out pubKey, out priKey, seed);

        //var keys = SubstrateNetApi..GenerateKeypairFromSeed(BytesToHexString(seed));

        Secret.text = $"[{priKey.Length}] {BytesToHexString(priKey, true)}";
        PublicKey.text = $"[{pubKey.Length}] {BytesToHexString(pubKey, true)}";

        Debug.Log($"PUB: {BytesToHexString(pubKey, true)}, PRI: {BytesToHexString(priKey, true)}");

        var msg = "Test sign me!";
        var msgBytes = Encoding.UTF8.GetBytes(msg);
        Debug.Log($"MSG: {msg} '{BytesToHexString(msgBytes)}'");


        var signedBytes = Ed25519.Sign(msgBytes, priKey);

        var test = Ed25519.Verify(signedBytes, msgBytes, pubKey);

        Debug.Log($"MSG (signed): {BytesToHexString(signedBytes)}");

        Debug.Log($"VERIFY (signed): {test}");

        Message.text = test ? "Successful 'Sign/Verify' Test!" : "Failed 'Sign/Verify' Test!";

        Message.color = test ? Color.green : Color.red;


    }

    public string BytesToHexString(byte[] bytes, bool prefixed = false)
    {
        return $"{(prefixed? "0x":"")}{BitConverter.ToString(bytes).Replace("-", string.Empty)}";
    }
}
