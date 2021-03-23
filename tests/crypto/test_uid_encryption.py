import pytest

from signal_groups.crypto.sho import Sho
from signal_groups.crypto.errors import ZkGroupException
from signal_groups.crypto.uid_struct import UidStruct
from signal_groups.crypto.uid_encryption import (
    KeyPair,
    SystemParams,
    PublicKey,
    Ciphertext,
)

from tests.constants import TEST_16, TEST_32


def test_uid_encrypt():
    TEST_32_1 = b"123456789010111213141516"
    transcript = Sho(b"Test_Uid_Encryption", TEST_32_1)

    SystemParams.generate().serialize() == SystemParams.get_hardcoded().serialize()

    key_pair = KeyPair.derive_from(transcript)

    key_pair_bytes = key_pair.serialize()
    with pytest.raises(ValueError, match="cannot deserialize"):
        KeyPair.deserialize(key_pair_bytes[0 : len(key_pair_bytes) - 1])
    key_pair2 = KeyPair.deserialize(key_pair_bytes)
    assert key_pair_bytes == key_pair2.serialize()

    uid = UidStruct(TEST_16)
    ciphertext = key_pair.encrypt(uid)

    ciphertext_bytes = ciphertext.serialize()
    assert len(ciphertext_bytes) == 64
    ciphertext2 = Ciphertext.deserialize(ciphertext_bytes)
    assert ciphertext.serialize() == ciphertext2.serialize()

    plaintext = key_pair.decrypt(ciphertext2)

    assert plaintext.serialize() == uid.serialize()
