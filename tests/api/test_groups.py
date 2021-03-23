from signal_groups.api import groups

from tests.constants import TEST_32


def test_GroupSecretParams_serialization():
    test_params = groups.GroupSecretParams.generate(TEST_32)

    serialized_obj = test_params.serialize()
    deserialized_obj = groups.GroupSecretParams.deserialize(serialized_obj)

    assert serialized_obj == deserialized_obj.serialize()


def test_GroupPublicParams_serialization():
    test_secret_params = groups.GroupSecretParams.generate(TEST_32)
    test_public_params = test_secret_params.get_public_params()

    serialized_obj = test_public_params.serialize()
    deserialized_obj = groups.GroupPublicParams.deserialize(serialized_obj)

    assert serialized_obj == deserialized_obj.serialize()
