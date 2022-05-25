# teltools
# Telugu transliteration tools

## Prerequisites
Install Rust in the platform of your choice and run cargo to build and execute like below

### Transliterate from English to Telugu
#### Example English input in in.txt captured in out.tel
```
om gaM gaNapatayE namaH ||
om shrI mahA saraswatyai namaH ||
om shrI gurubhyOH namaH ||
om namO bhagavatE vAsudEvAya ||
om namO bhagavatE rudrAya ||
om namO bhagavatE sharavaNabhavAya ||
om namO bhagavatE vE~NkaTEshAya ||
om shrI mAtrE namaH ||
hariH om ||
```

#### Example command invocation
`$ cargo run --bin trans in.txt > out.tel`

### Transliterate from Telugu to Devanagari
#### Example Telugu input in in.tel captured in out.hin
```
ॐ గం గణపతయే నమః ॥
ॐ శ్రీ మహా సరస్వత్యై నమః ॥
ॐ శ్రీ గురుభ్యోః నమః ॥
ॐ నమో భగవతే వాసుదేవాయ ॥
ॐ నమో భగవతే రుద్రాయ ॥
ॐ నమో భగవతే శరవణభవాయ ॥
ॐ నమో భగవతే వేఙ్కటేశాయ ॥
ॐ శ్రీ మాత్రే నమః ॥
హరిః ॐ ॥
```

#### Example command invocation
`$ cargo run --bin tel2hin in.tel > out.hin`

### Transliterate from Devanagari to Telugu
#### Example Devanagari input in in.hin captured in out.tel
```
ॐ गं गणपतये नमः ॥
ॐ श्री महा सरस्वत्यै नमः ॥
ॐ श्री गुरुभ्योः नमः ॥
ॐ नमो भगवते वासुदेवाय ॥
ॐ नमो भगवते रुद्राय ॥
ॐ नमो भगवते शरवणभवाय ॥
ॐ नमो भगवते वेङ्कटेशाय ॥
ॐ श्री मात्रे नमः ॥
हरिः ॐ ॥
```

#### Example command invocation
`$ cargo run --bin hin2tel in.hin > out.tel`
