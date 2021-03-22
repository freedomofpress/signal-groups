from signal_groups.crypto import profile_key_commitment, profile_key_struct

from tests.constants import TEST_16, TEST_32


def test_SystemParams_serialization():
    test_params = profile_key_commitment.SystemParams.generate()

    serialized_obj = test_params.serialize()
    deserialized_obj = profile_key_commitment.SystemParams.deserialize(serialized_obj)

    assert serialized_obj == deserialized_obj.serialize()


def test_commitment():
    profile_key = profile_key_struct.ProfileKeyStruct(TEST_32, TEST_16)

    c1 = profile_key_commitment.CommitmentWithSecretNonce(profile_key, TEST_16)
    c2 = profile_key_commitment.CommitmentWithSecretNonce(profile_key, TEST_16)

    assert c1.serialize() == c2.serialize()
