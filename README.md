<header>
<img src="https://raw.githubusercontent.com/uzinfocom-org/website/main/src/images/logo.svg" alt="logo" height="100" align="left">
<h1 style="display: inline">Vicardi</h1>

Serde serializatsiya va deserializatsiyasi yordamida qilingan VCard JSON generatori

[![GitHub top language](https://img.shields.io/github/languages/top/uzinfocom-org/vicardi?style=flat-square&logo=github)](https://github.com/uzinfocom-org/vicardi)
[![Chat](https://img.shields.io/badge/Chat-grey?style=flat-square&logo=telegram)](https://t.me/xinuxuz)
[![Test CI](https://github.com/uzinfocom-org/vicardi/actions/workflows/test.yml/badge.svg)](https://github.com/uzinfocom-org/vicardi/actions/workflows/test.yml)

</header>

## Haqida

Bizning CCTLD tomonidan ishlab chiqilgan sistema ICAN servislari bilan muloqot qilish uchun JCard (VCardArray) formatini ishlatadi. Afsuski,
VCardArray generatsion biron kutubxona mavjud bo'lmaganligi sababli o'zimizning kutubxonamizni ishlab chiqdik va ushbu kutubxonani O'zbekistondagi
.uz TLD domenlarini ishlashini ham tez, ham nustahkam qilib RDAP tizimini yaratishda ishlatdik.

> Ushbu kutubxona RFC7483 standartiga binoan ishlab chiqilgan. Ko'proq ma'lumotlar uchun shu yerga kiring:
> https://datatracker.ietf.org/doc/html/rfc7483

## Qulayliklar

- Tayyor "binding"lar
- Serde yordamida serializatsiyadan o'tgan `struct` lar
- (Yordamchi macroslar va shu kabi qulayliklar keyinchalik qo'shish niyat qilingan)

> Bu proyekt hozir sinov bosqichidan o'tmoqda. Agarda bironta xatolikka duchor
> bo'lsangiz, xatolik haqida [e'lon](https://github.com/uzinfocom-org/vicardi/issues/new)
> qoldirishni unutmang.

## O'rnatish

Ushbu qatorni Cargo.toml faylingizga joylashtiring:

```toml
[dependencies]
vicardi = "0.1.1"
```

## Litsenziya
    
Ushbu kutubxona GPL-3.0 litsenziya ostida distributsiya qilinadi. Ko'proq ma'lumot uchun [LICENSE](./LICENSE) ko'zdan kechiring!
