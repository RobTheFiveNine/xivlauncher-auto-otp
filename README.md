<div align="center">
    <img src=".github/lalafell.png" />
    <h1>XIVLauncher Auto OTP</h1>
</div>

## What is XIVLauncher Auto OTP?
It is a tool for generating OTP tokens for use with XIVLauncher on the Steam Deck to prevent the need to type anything when logging into Final Fantasy XIV.

To see a demonstration of how this works, check out [This YouTube Video](https://www.youtube.com/watch?v=-JH7yQgBsx0).

## Why is this a thing?
I personally find it difficult to use the text entry on the Steam Deck, XIVLauncher allows you to save your password to avoid having to type this out every time, however, it still requires a 2FA token be entered. This tool will automatically generate and pass the token to XIVLauncher when launched through the Steam Deck.

## Is this safe?
The answer to this depends on your risk appetite. The tool itself is by no means unsafe, however, much like when you save your password in XIVLauncher - this will result in your 2FA configuration being stored on the same device; some what defeating the main purpose of having 2FA (that being requiring a second device in the authentication process)

In practical terms, this means that if someone were able to steal your Steam Deck, they could in theory compromise your account, assuming you don't change your password / register a new 2FA token before they troll through the device.

Likewise, if someone were to compromise the device remotely and be able to access the file system, they could in theory steal the credentials (though, if someone has compromised the user session remotely, it would be trivial for them to capture this data and use it to compromise the account without this tool installed).

Although the protection that 2FA provides against things such as brute force attacks or compromised passwords is unaffected, it is important that you understand the risk introduced by doing this should your Steam Deck be stolen by someone who understands what the 2FA key / secret is before you can remove it from your Square Enix account.

## Setup guide
1. Ensure XIVLauncher is installed on your Steam Deck as per [The Official Instructions](https://goatcorp.github.io/faq/steamdeck.html)
2. Enable the OTP app feature in the settings screen of XIVLauncher (see the [Authenticator App Guide](https://goatcorp.github.io/faq/mobile_otp#enabling-the-otp-app-feature-in-xivlauncher) for more information)
3. Switch to Desktop Mode on the Steam Deck via the Power menu (the same way as you did to install XIVLauncher)
4. Download [The Latest Release](https://github.com/RobTheFiveNine/xivlauncher-auto-otp/releases/latest) of XIVLauncher Auto OTP and make note of where you stored it. We will assume it has been saved to `~/.xlcore/xivlauncher-auto-otp`.
5. Open a terminal window and ensure the file is executable by running: `chmod +x ~/.xlcore/xivlauncher-auto-otp` (replacing the path if you stored the file somewhere else)
6. In Steam's Desktop mode, select `Add a Game` and choose `Add a Non-Steam Game`.
7. In the window that pops up, click the `Browse` button and navigate to `~/.xlcore/xivlauncher-auto-otp` and select it (if you can't see the file, make sure to choose `All Files` in the filter drop down). After selecting the file, click `Add Selected Programs`
8. Find the newly added program in your games list, right click it and open the properties window.
9. In the `Launch Options` setting, copy and paste your OTP secret (see [How do I find my OTP secret?](#how-do-i-find-my-otp-secret) for more information on this).

You are now setup! To use the tool:

 - Launch XIVLauncher as you normally would on your Steam Deck.
 - When prompted for your 2FA code, hit the Steam button to return to your library list
 - Choose to launch XIVLauncher Auto OTP, the tool will launch briefly and exit after sending the token to XIVLauncher
 - At this point you should be automatically returned to XIVLauncher where it will be logging you into Final Fantasy XIV. If not returned straight back to XIVLauncher, just hit the Steam button and navigate back to it in your open apps at the top of the menu


## How do I find my OTP secret?
The OTP secret (also known as the "seed" or "key") is a 32 character long string that determines what tokens are generated for you by the application. In order for this tool to work, it needs to be able to generate tokens for you the same way your regular 2FA application does when you are generating tokens to login on the website or on other platforms.

How you find the OTP secret will depend on what application you are using to generate your 2FA tokens. Lots of applications (such as Google's own, Authy and others) will not allow you to retrieve the seed used to setup the token.

Some apps, such as KeePassXC, will allow you to see the `otpauth` URI that will contain the encoded secret (in KeePassXC this can be found in the `Advanced > Additional attributes > otp` entry, you will see a URI that contains a `secret` parameter like this: `otpauth://totp/Square%20Enix:YOUR_USERNAME?secret=YOUR_2FA_SECRET_HERE&period=30&digits=6&issuer=Square%20Enix`)

If you are unable to extract the secret from your current 2FA setup, your only way forward will be to remove the existing 2FA from your account, and set it up again, but when setting it back up, choosing to enter the key (aka the secret) manually rather than using the QR code.

For instructions on setting up an authenticator without a QR code, see [https://www.square-enix-games.com/en_GB/seaccount/otp/authenticator.html](https://www.square-enix-games.com/en_GB/seaccount/otp/authenticator.html); specifically, the section titled `Registration for Google Authenticator (Authentication Key Entry)`.

When setting your 2FA back up this way, make note of the key that you are entering into your Google Authenticator app, as this will be the value we use as the secret in XIVLauncher Auto OTP.

Most (if not all) Google Authenticator apps should support adding new tokens by entering a key manually.

## How do I compile from source?
A compiled version of the tool is avilable in the releases section, however, if you would rather compile it yourself:

1. Install [Rust](https://www.rust-lang.org/) on your system
2. (Optional, but likely required) Ensure the `libssl-dev` package is installed using whichever package manager is provided with your system
3. Clone the repository: `git clone https://github.com/RobTheFiveNine/xivlauncher-auto-otp.git`
4. Enter the `src` directory: `cd xivlauncher-auto-otp/src`
5. Build the source code in release mode: `cargo build -r`

Providing there are no errors, the compiled binary should now be available at `xivlauncher-auto-otp/build/x86_64-unknown-linux-gnu/release/xivlauncher-auto-otp`

## License
This Source Code Form is subject to the terms of the Mozilla Public License, v. 2.0. If a copy of the MPL was not distributed with this file, You can obtain one at [https://mozilla.org/MPL/2.0/](https://mozilla.org/MPL/2.0/).
