# teltools
# Telugu transliteration tools

## Prerequisites
Install Rust in the platform of your choice and run cargo to build and execute like below

### Transliterate from English to Telugu
#### Example English input in in.txt captured in out.tel
```
om gaM gaNapatayE namaH || <br />
om shrI mahA saraswatyai namaH || <br />
om shrI gurubhyOH namaH || <br />
om namO bhagavatE vAsudEvAya || <br />
om namO bhagavatE rudrAya || <br />
om namO bhagavatE sharavaNabhavAya || <br />
om namO bhagavatE vE~NkaTEshAya || <br />
om shrI mAtrE namaH || <br />
hariH om ||
```

#### Example command invocation
`$ cargo run --bin trans in.txt > out.tel`

### Transliterate from Telugu to Devanagari
#### Example Telugu input in in.tel captured in out.hin
```
ॐ గం గణపతయే నమః ॥ <br />
ॐ శ్రీ మహా సరస్వత్యై నమః ॥ <br />
ॐ శ్రీ గురుభ్యోః నమః ॥ <br />
ॐ నమో భగవతే వాసుదేవాయ ॥ <br />
ॐ నమో భగవతే రుద్రాయ ॥ <br />
ॐ నమో భగవతే శరవణభవాయ ॥ <br />
ॐ నమో భగవతే వేఙ్కటేశాయ ॥ <br />
ॐ శ్రీ మాత్రే నమః ॥ <br />
హరిః ॐ ॥ <br />
```

#### Example command invocation
`$ cargo run --bin tel2hin in.tel > out.hin`

### Transliterate from Devanagari to Telugu
#### Example Devanagari input in in.hin captured in out.tel
```
ॐ गं गणपतये नमः ॥<br />
ॐ श्री महा सरस्वत्यै नमः ॥<br />
ॐ श्री गुरुभ्योः नमः ॥<br />
ॐ नमो भगवते वासुदेवाय ॥<br />
ॐ नमो भगवते रुद्राय ॥<br />
ॐ नमो भगवते शरवणभवाय ॥<br />
ॐ नमो भगवते वेङ्कटेशाय ॥<br />
ॐ श्री मात्रे नमः ॥<br />
हरिः ॐ ॥
```

#### Example command invocation
`$ cargo run --bin hin2tel in.hin > out.tel`
