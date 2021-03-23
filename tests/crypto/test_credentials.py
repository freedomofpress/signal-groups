import pytest

from signal_groups.crypto.errors import ZkGroupException
from signal_groups.crypto import credentials, uid_struct, proofs, sho

from tests.constants import TEST_16, TEST_32


def test_SystemParams_serialization():
    test_params = credentials.SystemParams.generate()

    serialized_obj = test_params.serialize()
    deserialized_obj = credentials.SystemParams.deserialize(serialized_obj)

    assert serialized_obj == deserialized_obj.serialize()


def test_system():
    test_params = credentials.SystemParams.generate()

    assert (
        test_params.serialize() == credentials.SystemParams.get_hardcoded().serialize()
    )


def test_mac():
    transcript = sho.Sho(b"Test_Credentials", b"")
    NUM_AUTH_CRED_ATTRIBUTES = 3
    keypair = credentials.KeyPair.generate(transcript, NUM_AUTH_CRED_ATTRIBUTES)

    uid_bytes = TEST_16
    redemption_time = 37
    uid = uid_struct.UidStruct(uid_bytes)
    credential = keypair.create_auth_credential(uid, redemption_time, transcript)
    proof = proofs.AuthCredentialIssuanceProof(
        keypair,
        credential,
        uid,
        redemption_time,
        transcript,
    )

    public_key = keypair.get_public_key()

    # should not raise exception
    proof.verify(public_key, credential, uid, redemption_time)

    # should fail if redeemed too late
    with pytest.raises(ZkGroupException):
        proof.verify(public_key, credential, uid, 1000000)

    keypair_bytes = keypair.serialize()
    keypair2 = credentials.KeyPair.deserialize(keypair_bytes)
    assert keypair_bytes == keypair2.serialize()

    public_key_bytes = public_key.serialize()
    public_key2 = credentials.PublicKey.deserialize(public_key_bytes)
    assert public_key_bytes == public_key2.serialize()

    mac_bytes = credential.serialize()
    mac2 = credentials.AuthCredential.deserialize(mac_bytes)
    assert mac_bytes == mac2.serialize()
