from signal_groups.crypto import (
    profile_key_credential_request,
    profile_key_struct,
    profile_key_commitment,
    sho,
)

from tests.constants import TEST_16, TEST_32


def test_request():
    transcript = sho.Sho(b"Test_Profile_Key_Credential_Request", b"")

    blind_key_pair = profile_key_credential_request.KeyPair.generate(transcript)

    profile_key = profile_key_struct.ProfileKeyStruct(TEST_32, TEST_16)
    profile_key_commit = profile_key_commitment.CommitmentWithSecretNonce(
        profile_key, TEST_16
    )

    ciphertext = blind_key_pair.encrypt(profile_key, transcript)
