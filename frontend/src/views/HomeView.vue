<script setup lang="ts">
import { GrpcWebFetchTransport } from '@protobuf-ts/grpcweb-transport';
import { VotingClient } from '@/protos/voting.client';
import { ref } from 'vue';

let transport = new GrpcWebFetchTransport({
    baseUrl: 'http://127.0.0.1:8009',
    format: 'binary',
});

const confirmation = ref<string | null>(null);

const url = ref<string | null>(null);

const vote = async () => {
    if (!url.value) {
        alert('bugger off');
        return;
    }

    let client = new VotingClient(transport);

    try {
        let { response } = await client.vote({ url: url.value as string, vote: 1 });
        confirmation.value = response.confirmation;
    } catch (error: any) {
        if (error.RpcError) {
            alert(error.message);
        }

        console.log(error);
    } finally {
        url.value = null;
    }
};

const list = async () => {
    let client = new VotingClient(transport);

    try {
        let { response } = await client.index({});
        console.log(response);
    } catch (error: any) {
        if (error.RpcError) {
            alert(error.RpcError);
        }

        console.log(error);
    }
};

const get = async () => {
    let client = new VotingClient(transport);

    try {
        let { response } = await client.get({
            id: 1,
        });

        console.log(response.vote);
    } catch (error: any) {
        if (error.RpcError) {
            alert(error.RpcError);
        }

        console.log(error);
    }
};
</script>

<template>
    <h1>Vote</h1>
    <form @submit.prevent="vote">
        <div>
            <input v-model="url" />
        </div>
        <div>
            <button type="submit">Submit</button>
        </div>
        <p>{{ confirmation }}</p>
    </form>
    <hr />
    <div>
        <button @click="list">List</button>
    </div>
    <hr />
    <div>
        <button @click="get">Get</button>
    </div>
</template>
