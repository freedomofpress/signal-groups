from signal_groups.crypto import profile_key_commitment


def test_SystemParams_serialization():
    test_params = profile_key_commitment.SystemParams.generate()

    serialized_obj = test_params.serialize()
    deserialized_obj = profile_key_commitment.SystemParams.deserialize(serialized_obj)

    assert serialized_obj == deserialized_obj.serialize()
