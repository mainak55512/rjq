<h1 align="center">Welcome to rjq 👋</h1>
<p>
  <a href="github.com/mainak55512/rjq" target="_blank">
    <img alt="Documentation" src="https://img.shields.io/badge/documentation-yes-brightgreen.svg" />
  </a>
  <a href="#" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  </a>
</p>

> Simple and fast JSON filtering tool written in Rust.

### 🏠 [Homepage](https://github.com/mainak55512/rjq)

## Usage

```sh
rjq --load="<file path>" --query="<query string>" --params="<comma separated parameter names>"
```
or,

```sh
stto --json cpython | rjq --query="<query string>" --params="<comma separated parameter names>"
```

Demo data.json:

```sh
[
  {
    "_id": "88L33FM4VQBB1QYH",
    "name": "Eleonore Kendall",
    "dob": "2021-04-13",
    "address": {
      "street": "3137 Stich Avenue",
      "town": "Swanley",
      "postode": "CR45 9NE"
    },
    "telephone": "+49-2424-456-409",
    "pets": [
      "Murphy",
      "Oliver"
    ],
    "score": 2.4,
    "email": "etheleneweston@shuttle.com",
    "url": "http://www.consultant.com",
    "description": "mobiles besides deputy australian picnic germany collectables gmc necessity both webcams testing set continuity bread candle drivers p icon alone",
    "verified": true,
    "salary": 24057
  },
  {
    "_id": "OQGS24A7RF6D118C",
    "name": "Shona Breen",
    "dob": "2017-08-13",
    "address": {
      "street": "6515 Camberley",
      "town": "Fortrose",
      "postode": "SS64 6WU"
    },
    "telephone": "+42-1296-691-224",
    "pets": [
      "Lilly",
      "Penny"
    ],
    "score": 8,
    "email": "gerald-love@para.rome.it",
    "url": "http://district.com",
    "description": "collective submitted samuel del kenya fi wordpress etc worked realize enzyme ethernet assured championships preferred examples virtual bluetooth urw trust",
    "verified": false,
    "salary": 10760
  },
  {
    "_id": "YI818QFFYNA6AZUH",
    "name": "Dot Milton",
    "dob": "2015-03-01",
    "address": {
      "street": "2151 Kenstford",
      "town": "Coldstream",
      "postode": "WN26 1ZO"
    },
    "telephone": "+45-8699-662-747",
    "pets": [
      "Murphy",
      "Bear"
    ],
    "score": 2.4,
    "email": "karla.harrington4058@relation.com",
    "url": "https://www.minolta.com",
    "description": "segments programmes tulsa acre placing prix awarded senators main trim played italiano difficulties cab customers granny document pf exceptions attractions",
    "verified": true,
    "salary": 54912
  },
  ...
]
```

rjq query:

```sh
rjq --load="data.json" --query="salary < 15000 && score < 2.0 && verified = false" --params="name, address.town, pets"
```

output:

```sh
[
  {
    "name": "Rosaura Thurston",
    "pets": [
      "BatMan",
      "Leo"
    ],
    "town": "Matlock"
  },
  {
    "name": "Bernardina Bateman",
    "pets": [
      "sox",
      "Stella"
    ],
    "town": "Petersfield"
  },
  {
    "name": "Miquel Cranford",
    "pets": [
      "Oscar",
      "Oliver"
    ],
    "town": "Olney"
  },
  {
    "name": "Hanna Scherer",
    "pets": [
      "Sammy",
      "Penny"
    ],
    "town": "Greenhill"
  },
  {
    "name": "Arcelia Woodcock",
    "pets": [
      "boo",
      "Lexi"
    ],
    "town": "Kendal"
  },
  {
    "name": "Kayce Gable",
    "pets": [
      "Leo",
      "Murphy"
    ],
    "town": "Otley"
  },
  ...
]
```

## Benchmark

![benchmark](./media/rjq_benchmark.png)

## Author

👤 **Mainak Bhattacharjee**

* Github: [@mainak55512](https://github.com/mainak55512)

## 🤝 Contributing

Contributions, issues and feature requests are welcome!<br />Feel free to check [issues page](https://github.com/mainak55512/rjq/issues). 

## Show your support

Give a ⭐️ if this project helped you!
