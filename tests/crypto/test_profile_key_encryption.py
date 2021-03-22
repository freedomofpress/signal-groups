from signal_groups.common import Sho
from signal_groups.crypto.profile_key_struct import ProfileKeyStruct
from signal_groups.crypto.profile_key_encryption import (
    Ciphertext,
    KeyPair,
    SystemParams,
)

# from tests.constants import TEST_32_1
from tests.constants import TEST_16, TEST_32


def test_profile_key_encryption():
    TEST_32_1 = b"123456789010111213141516"
    transcript = Sho(b"Test_Profile_Key_Encryption", TEST_32_1)

    assert (
        SystemParams.generate().serialize() == SystemParams.get_hardcoded().serialize()
    )

    key_pair = KeyPair.derive_from(transcript)
    key_pair_bytes = key_pair.serialize()
    key_pair2 = KeyPair.deserialize(key_pair_bytes)
    assert key_pair_bytes == key_pair2.serialize()

    profile_key = ProfileKeyStruct(TEST_32, TEST_16)
    ciphertext = key_pair.encrypt(profile_key)

    ciphertext_bytes = ciphertext.serialize()
    assert len(ciphertext_bytes) == 64
    ciphertext2 = Ciphertext.deserialize(ciphertext_bytes)
    assert ciphertext_bytes == ciphertext2.serialize()

    plaintext = key_pair.decrypt(ciphertext2, TEST_16)
    assert plaintext.serialize() == profile_key.serialize()
