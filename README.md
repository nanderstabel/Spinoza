# Spinoza

I was quite ambitious when working on this assignment, which made me spend more time on it than the suggested time spend of 2-3 hours. However, I had a lot of fun working on it. All and all I think I have spend a bit over 6 hours in total. 

My idea was to create several identities that together tell a story about the life of the philosopher Baruch de Spinoza. The main character of the story of course would be Spinoza himself. In the beginning of the story his identity would be created and he would receive a verifiable credential (VC) (symbolizing his lifespan) from the "17th Century" welcoming him to the beginning of his life.

During his life several more identities would interact with him and create and revoke multiple instances of VC. Most notably A VC from the Jewish Community (at the time) would be revoked in July of 1656 (cherem) because of Spinoza's ideas and writings.

In this current version only the creation of the two identities and the creation, presentation and verification of their first VC is implemented. 

Furthermore it was my intention to add a simple frontend as well that would show how the identities and VC's would get created in a more interactive way of telling the story.

Even though I did not manage to complete my initial ideas in the given timeframe, I am still quite content with the outcome of this assignment. Some sidenotes:
- I realized I do not know how to apply unit testing in the Axum framework that I have used, hence why tests are completely absent as of now. I have looked into the Mockall crate and will probably get familiar with that at some point.
- Subsequently, I did try to apply some proper error-handling but there is definitely room for improvement on this part as well.
- All stronghold file locations, passwords and fragments are the same for all identities as of now (for simplicity).

Below you can find a demonstration of how I created two identities on my local machine and the creation, presentation, 
and verification of a verifiable credential between the two identities (might add a script to demonstrate this at a later point).

To start the server:
```
cd backend
cargo run
```

In a new terminal (in the same `/backend` directory), execute the following curl commands. WARNING: you will not get the same output, since you will receive different did's. I might add a script for better demonstration later.

// Create first account for Baruch.
REQUEST:
```
curl -H 'Content-Type: application/json' \
     -H 'stronghold: ./canttouch.this' \
     -H 'password: hold-on' \
     -X POST \
     http://127.0.0.1:3000/api/v1/account/create
```
RESPONSE:
"did:iota:21SykGgJyXZzAidzeqf3RVyqNNa9rvK1ekAC47cj7EnW"


// Create second account for 17th Century (using same stronghold path and password for simplicity).
REQUEST:
```
curl -H 'Content-Type: application/json' \
     -H 'stronghold: ./canttouch.this' \
     -H 'password: hold-on' \
     -X POST \
     http://127.0.0.1:3000/api/v1/account/create
```
RESPONSE:
"did:iota:7asn3NTRAT655tgYMrxZDvL76bL4NEhFckkJRyAz1ytS"


// To check if account creation worked: commit a read for Baruch
REQUEST:
```
curl -H 'Content-Type: application/json' \
     -H 'stronghold: ./canttouch.this' \
     -H 'password: hold-on' \
     -H 'did: did:iota:21SykGgJyXZzAidzeqf3RVyqNNa9rvK1ekAC47cj7EnW' \
     -X POST \
     http://127.0.0.1:3000/api/v1/account/read | json_pp
```
RESPONSE:
{
   "doc" : {
      "capabilityInvocation" : [
         {
            "controller" : "did:iota:21SykGgJyXZzAidzeqf3RVyqNNa9rvK1ekAC47cj7EnW",
            "id" : "did:iota:21SykGgJyXZzAidzeqf3RVyqNNa9rvK1ekAC47cj7EnW#sign-0",
            "publicKeyMultibase" : "z4bKG1TqvCnnC7m8ccqhA4df8CNYSffADv1ewPQQbFh14",
            "type" : "Ed25519VerificationKey2018"
         }
      ],
      "id" : "did:iota:21SykGgJyXZzAidzeqf3RVyqNNa9rvK1ekAC47cj7EnW"
   },
   "meta" : {
      "created" : "2023-01-20T14:59:33Z",
      "updated" : "2023-01-20T14:59:33Z"
   },
   "proof" : {
      "signatureValue" : "53nzeTmDSkDpdQRRpG4DNd5b1daAGYccYjxfcn4EwFygiQRWdj6DFgP7h5knu8VCzFmviFke6FDPYavNruZD2JYp",
      "type" : "JcsEd25519Signature2020",
      "verificationMethod" : "did:iota:21SykGgJyXZzAidzeqf3RVyqNNa9rvK1ekAC47cj7EnW#sign-0"
   }
}

// Update second account for 17th Century which adds a method to its account.
REQUEST:
```
curl -H 'Content-Type: application/json' \
     -H 'stronghold: ./canttouch.this' \
     -H 'password: hold-on' \
     -H 'did: did:iota:7asn3NTRAT655tgYMrxZDvL76bL4NEhFckkJRyAz1ytS' \
     -X POST \
     http://127.0.0.1:3000/api/v1/account/update | json_pp
```
RESPONSE:
{
   "doc" : {
      "capabilityInvocation" : [
         {
            "controller" : "did:iota:7asn3NTRAT655tgYMrxZDvL76bL4NEhFckkJRyAz1ytS",
            "id" : "did:iota:7asn3NTRAT655tgYMrxZDvL76bL4NEhFckkJRyAz1ytS#sign-0",
            "publicKeyMultibase" : "zD4C4nKoZUZFdtcSCbaGg9hRpm3D2RXkwuJFQmWTVWmP6",
            "type" : "Ed25519VerificationKey2018"
         }
      ],
      "id" : "did:iota:7asn3NTRAT655tgYMrxZDvL76bL4NEhFckkJRyAz1ytS",
      "verificationMethod" : [
         {
            "controller" : "did:iota:7asn3NTRAT655tgYMrxZDvL76bL4NEhFckkJRyAz1ytS",
            "id" : "did:iota:7asn3NTRAT655tgYMrxZDvL76bL4NEhFckkJRyAz1ytS#issuerKey",
            "publicKeyMultibase" : "zBsGq3ewXk5TR6Ks1fA55tWuEqrBWWE4tza7VLT356ypb",
            "type" : "Ed25519VerificationKey2018"
         }
      ]
   },
   "meta" : {
      "created" : "2023-01-20T15:02:10Z",
      "previousMessageId" : "226071454c581c24cbfa38a350102923106aa13abdfda1bb028a38e8c861de99",
      "updated" : "2023-01-20T15:09:08Z"
   },
   "proof" : {
      "signatureValue" : "2jtSdDxP7SFn4etFMW6SPa2JFsudsXDJgT7rLs5gwfEX4zoGoQF3kZpq1wmCns3TVppz2yWuf3HuTpNbfmHKghYq",
      "type" : "JcsEd25519Signature2020",
      "verificationMethod" : "did:iota:7asn3NTRAT655tgYMrxZDvL76bL4NEhFckkJRyAz1ytS#sign-0"
   }
}

// Add the same method to Baruch's account.
REQUEST:
```
curl -H 'Content-Type: application/json' \
     -H 'stronghold: ./canttouch.this' \
     -H 'password: hold-on' \
     -H 'did: did:iota:21SykGgJyXZzAidzeqf3RVyqNNa9rvK1ekAC47cj7EnW' \
     -X POST \
     http://127.0.0.1:3000/api/v1/account/update | json_pp
```
RESPONSE:
{
   "doc" : {
      "capabilityInvocation" : [
         {
            "controller" : "did:iota:21SykGgJyXZzAidzeqf3RVyqNNa9rvK1ekAC47cj7EnW",
            "id" : "did:iota:21SykGgJyXZzAidzeqf3RVyqNNa9rvK1ekAC47cj7EnW#sign-0",
            "publicKeyMultibase" : "z4bKG1TqvCnnC7m8ccqhA4df8CNYSffADv1ewPQQbFh14",
            "type" : "Ed25519VerificationKey2018"
         }
      ],
      "id" : "did:iota:21SykGgJyXZzAidzeqf3RVyqNNa9rvK1ekAC47cj7EnW",
      "verificationMethod" : [
         {
            "controller" : "did:iota:21SykGgJyXZzAidzeqf3RVyqNNa9rvK1ekAC47cj7EnW",
            "id" : "did:iota:21SykGgJyXZzAidzeqf3RVyqNNa9rvK1ekAC47cj7EnW#issuerKey",
            "publicKeyMultibase" : "zGqfbwr97Kk3Tb7MKb7if9BHvSzy5VxVxnrnp7yHcApDP",
            "type" : "Ed25519VerificationKey2018"
         }
      ]
   },
   "meta" : {
      "created" : "2023-01-20T14:59:33Z",
      "previousMessageId" : "c87e9705c90096ad2d87cc32092f6d15f3ba092a8401f0b80932f983ce3b7c1c",
      "updated" : "2023-01-20T15:10:48Z"
   },
   "proof" : {
      "signatureValue" : "34nQVbcqLaxpYMveMpne6TFPnrfdjiY6XXBGgMQZHnuvcWWsNLeZWVwXtL5PNeGnT72LTm6R3tMhUyvCm36cuiiE",
      "type" : "JcsEd25519Signature2020",
      "verificationMethod" : "did:iota:21SykGgJyXZzAidzeqf3RVyqNNa9rvK1ekAC47cj7EnW#sign-0"
   }
}

// Let the 17th Century account (issuer) create a new Credential for Baruch (holder)
REQUEST:
```
curl -H 'Content-Type: application/json' \
     -H 'stronghold: ./canttouch.this' \
     -H 'password: hold-on' \
     -H 'issuer_did: did:iota:7asn3NTRAT655tgYMrxZDvL76bL4NEhFckkJRyAz1ytS' \
     -H 'holder_did: did:iota:21SykGgJyXZzAidzeqf3RVyqNNa9rvK1ekAC47cj7EnW' \
     -X POST \
     http://127.0.0.1:3000/api/v1/credential/create | json_pp
```
RESPONSE:
{
   "@context" : "https://www.w3.org/2018/credentials/v1",
   "credentialSubject" : {
      "id" : "did:iota:21SykGgJyXZzAidzeqf3RVyqNNa9rvK1ekAC47cj7EnW",
      "lifespan" : {
         "born" : "24-11-1632",
         "died" : "21-02-1677"
      },
      "name" : "Baruch"
   },
   "id" : "https://example.edu/birth-certificate/baruch-de-spinoza",
   "issuanceDate" : "2023-01-20T15:15:07Z",
   "issuer" : "did:iota:7asn3NTRAT655tgYMrxZDvL76bL4NEhFckkJRyAz1ytS",
   "proof" : {
      "signatureValue" : "5BCwReLMKYrb8ngywuQBSyWQV7tjm9SXMqnQp6JN74kqUWRBcTxnCPn6nm8d7bPbgX4PnCD88ALrYVYGVJivJqG7",
      "type" : "JcsEd25519Signature2020",
      "verificationMethod" : "did:iota:7asn3NTRAT655tgYMrxZDvL76bL4NEhFckkJRyAz1ytS#issuerKey"
   },
   "type" : [
      "VerifiableCredential",
      "LifeSpan"
   ]
}

// Let Baruch sign the new Credential and obtain a Presentation
REQUEST:
```
curl -H 'Content-Type: application/json' \
     -H 'stronghold: ./canttouch.this' \
     -H 'password: hold-on' \
     -H 'issuer_did: did:iota:7asn3NTRAT655tgYMrxZDvL76bL4NEhFckkJRyAz1ytS' \
     -H 'holder_did: did:iota:21SykGgJyXZzAidzeqf3RVyqNNa9rvK1ekAC47cj7EnW' \
     -X POST \
     http://127.0.0.1:3000/api/v1/credential/present -d @credential.json | json_pp
```
RESPONSE:
{
   "@context" : "https://www.w3.org/2018/credentials/v1",
   "holder" : "did:iota:21SykGgJyXZzAidzeqf3RVyqNNa9rvK1ekAC47cj7EnW",
   "proof" : {
      "signatureValue" : "2JfhsRg1v8vfnpPjXoVBtjk9cmypQLvnx58oZrSbTojXn5iQPckbhXLkjwjzgddMfMiVCKNg8UheAHnS5f3WpnHs",
      "type" : "JcsEd25519Signature2020",
      "verificationMethod" : "did:iota:21SykGgJyXZzAidzeqf3RVyqNNa9rvK1ekAC47cj7EnW#issuerKey"
   },
   "type" : "VerifiablePresentation",
   "verifiableCredential" : {
      "@context" : "https://www.w3.org/2018/credentials/v1",
      "credentialSubject" : {
         "id" : "did:iota:21SykGgJyXZzAidzeqf3RVyqNNa9rvK1ekAC47cj7EnW",
         "lifespan" : {
            "born" : "24-11-1632",
            "died" : "21-02-1677"
         },
         "name" : "Baruch"
      },
      "id" : "https://example.edu/birth-certificate/baruch-de-spinoza",
      "issuanceDate" : "2023-01-20T15:15:07Z",
      "issuer" : "did:iota:7asn3NTRAT655tgYMrxZDvL76bL4NEhFckkJRyAz1ytS",
      "proof" : {
         "signatureValue" : "5BCwReLMKYrb8ngywuQBSyWQV7tjm9SXMqnQp6JN74kqUWRBcTxnCPn6nm8d7bPbgX4PnCD88ALrYVYGVJivJqG7",
         "type" : "JcsEd25519Signature2020",
         "verificationMethod" : "did:iota:7asn3NTRAT655tgYMrxZDvL76bL4NEhFckkJRyAz1ytS#issuerKey"
      },
      "type" : [
         "VerifiableCredential",
         "LifeSpan"
      ]
   }
}

// Verify the Presentation
REQUEST:
```
curl -H 'Content-Type: application/json' \
     -H 'stronghold: ./canttouch.this' \
     -H 'password: hold-on' \
     -H 'issuer_did: did:iota:7asn3NTRAT655tgYMrxZDvL76bL4NEhFckkJRyAz1ytS' \
     -H 'holder_did: did:iota:21SykGgJyXZzAidzeqf3RVyqNNa9rvK1ekAC47cj7EnW' \
     -X POST \
     http://127.0.0.1:3000/api/v1/credential/verify -d @presentation.json | json_pp
```
RESPONSE:
"presentation: succesfully verified"

// Delete Baruch's account:
REQUEST:
```
curl -H 'Content-Type: application/json' \
     -H 'stronghold: ./canttouch.this' \
     -H 'password: hold-on' \
     -H 'did: did:iota:21SykGgJyXZzAidzeqf3RVyqNNa9rvK1ekAC47cj7EnW' \
     -X POST \
     http://127.0.0.1:3000/api/v1/account/delete | json_pp
```
RESPONSE:
"account: succesfully deleted"

// Delete 17th Century's account:
REQUEST:
```
curl -H 'Content-Type: application/json' \
     -H 'stronghold: ./canttouch.this' \
     -H 'password: hold-on' \
     -H 'did: did:iota:7asn3NTRAT655tgYMrxZDvL76bL4NEhFckkJRyAz1ytS' \
     -X POST \
     http://127.0.0.1:3000/api/v1/account/delete | json_pp
```
RESPONSE:
"account: succesfully deleted"
