# Cake blueprint

Simple Scrypto blueprint that shows how to combine different other blueprints by using a cake as an example.

In this version we are adding native assets like fungible and non fungible tokens to slice our cake in pieces.

## How to install

1. Install the latest Scrypto version and prepare your IDE as described in the official documentation.

2. Clone this repository

3. Run ```Scrypto build```

4. Go to the developer console and publish the package.

## For the fungible version

### Bake your cake (instantiate your component)

```
CALL_FUNCTION
  Address("package_tdx_2_1phdyct89z782wwnrwegkfzu695uusu56204czqydpucdawdfrm2xkq")
  "CakeFungible"
  "instantiate_cake_with_fungibles"
  true
  Decimal("123.45");
```

### Get a slice of cake
We take 200 XRD out of our account put it into a bucket, transfer the bucjet to the component and get a slide
of cake as well as our change back.

```
CALL_METHOD
  Address("account_tdx_2_12xf3455ks96n777zvsw8a5hvuk2cqzm58ktwrfuzrf6yqrl6ydv2h6")
  "withdraw"
  Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
  Decimal("200");
TAKE_FROM_WORKTOP
  Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
  Decimal("200")
  Bucket("xrd");
CALL_METHOD
  Address("component_tdx_2_1cr2h8a28multg6krsg33qpuautc3upmgrksasv039j9q6rcfuhy5d9")
  "buy_cake_slice"
  Bucket("xrd");
CALL_METHOD
  Address("account_tdx_2_12xf3455ks96n777zvsw8a5hvuk2cqzm58ktwrfuzrf6yqrl6ydv2h6")
  "deposit_batch"
  Expression("ENTIRE_WORKTOP");
```

## For the non fungible version

### Bake your cake (instantiate your component)
```
CALL_FUNCTION
  Address("package_tdx_2_1p4s3helm8r3ram0dnqfcrduewwzxk6t6cuvhn6fgrk4tjjydjfqxw2")
  "CakeNFT"
  "instantiate_cake_with_nft"
  true;
```

### Get a slice of cake
```
CALL_METHOD
  Address("account_tdx_2_12xf3455ks96n777zvsw8a5hvuk2cqzm58ktwrfuzrf6yqrl6ydv2h6")
  "withdraw"
  Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
  Decimal("200");
TAKE_FROM_WORKTOP
  Address("resource_tdx_2_1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxtfd2jc")
  Decimal("200")
  Bucket("xrd");
CALL_METHOD
  Address("component_tdx_2_1cqnk4ejgu7gtg5pu5w7jd25mw3zj0h9qdn8z2tj3af3gwztt4y5dk0")
  "get_cake_slice"
  Decimal("245.6")
  Decimal("670")
  "Cherry"
  Bucket("xrd");
CALL_METHOD
  Address("account_tdx_2_12xf3455ks96n777zvsw8a5hvuk2cqzm58ktwrfuzrf6yqrl6ydv2h6")
  "deposit_batch"
  Expression("ENTIRE_WORKTOP");
```