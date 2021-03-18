from signal_groups.crypto import credentials


def test_SystemParams_serialization():
    test_params = credentials.SystemParams.generate()

    serialized_obj = test_params.serialize()
    deserialized_obj = credentials.SystemParams.deserialize(serialized_obj)

    assert serialized_obj == deserialized_obj.serialize()
