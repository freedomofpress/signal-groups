import pytest

from signal_groups.crypto.sho import Sho
from signal_groups.crypto.errors import ZkGroupException
from signal_groups.crypto.signature import KeyPair

from tests.constants import TEST_32


def test_sig():
    TEST_32_1 = b"123456789010111213141516"
    transcript = Sho(b"Test_Signature", TEST_32_1)
    key_pair = KeyPair.generate(transcript)

    key_pair_bytes = key_pair.serialize()
    assert len(key_pair_bytes) == 64
    public_key_bytes = key_pair.get_public_key().serialize()
    assert len(public_key_bytes) == 32
    key_pair2 = KeyPair.deserialize(key_pair_bytes)
    assert key_pair_bytes == key_pair2.serialize()

    message = TEST_32_1

    signature = key_pair.sign(message, transcript)
    signature_1 = signature[:32]
    signature_2 = signature[32:]

    # should not raise exception
    key_pair2.get_public_key().verify(message, signature_1, signature_2)

    message = bytes([message[0] ^ 0x01]) + message[1:]

    with pytest.raises(ZkGroupException, match="SignatureVerificationFailure"):
        key_pair2.get_public_key().verify(message, signature_1, signature_2)
