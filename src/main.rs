use concrete::*;

fn main() -> Result<(), CryptoAPIError> {
    // encoders
    let encoder_input = Encoder::new(0., 16., 4, 1)?;
    let encoder_output = Encoder::new(1., 4., 2, 0)?;

    // secret keys
    let sk_rlwe = RLWESecretKey::new(&RLWE128_1024_1);
    let sk_in = LWESecretKey::new(&LWE128_630);
    let sk_out = sk_rlwe.to_lwe_secret_key();
    sk_out.save("sk_out.json").unwrap_or(());

    // bootstrapping key
    let bsk = LWEBSK::new(&sk_in, &sk_rlwe, 5, 3);
    bsk.save("bsk.json");

    // messages
    let message = 10.;

    // encode and encrypt
    let c1 = LWE::encode_encrypt(&sk_in, message, &encoder_input)?;

    let lookup_function = |x: f64| 
        if x > 8. {
            if x > 12. {
                4.
            }
            else{
                3.
            }
        } else {
            if x > 4. {
                2.
            }
            else {
                1.
            }
        };

    // bootstrap
    let c2 = c1.bootstrap_with_function(&bsk, lookup_function, &encoder_output)?;
    c2.save("ciphertext.json").unwrap_or(());

    // decrypt
    let output = c2.decrypt_decode(&sk_out)?;

    let rounded_output = output.round();

    println!("before bootstrap: {}, after bootstrap: {}", message, rounded_output);

    Ok(())
}
