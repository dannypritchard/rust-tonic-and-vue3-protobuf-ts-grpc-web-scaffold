<script setup lang="ts">
import { GrpcWebFetchTransport } from '@protobuf-ts/grpcweb-transport';
import { VotingClient } from '@/protos/voting.client';
import { ref } from 'vue';
import type { Vote } from '@/protos/voting';

let transport = new GrpcWebFetchTransport({
    baseUrl: 'http://127.0.0.1:8009',
    format: 'binary',
});

const list_result = ref<Vote[] | undefined>(undefined);
const get_result = ref<Vote | undefined>(undefined);
const vote_result = ref<Vote | undefined>(undefined);

const url = ref<string | null>(null);

const id = ref<number | null>(null);

const vote = async () => {
    if (!url.value) {
        alert('bugger off');
        return;
    }

    let client = new VotingClient(transport);

    try {
        let { response } = await client.vote({ url: url.value, vote: 1 });
        vote_result.value = response.vote;
    } catch (error: unknown) {
        if (error && typeof error === 'object' && "RpcError" in error) {
            alert(error.RpcError);
        }
    } finally {
        url.value = null;
    }
};

const list = async () => {
    let client = new VotingClient(transport);

    try {
        let { response } = await client.index({});
        list_result.value = response.votes;
    } catch (error: unknown) {
        if (error && typeof error === 'object' && "RpcError" in error) {
            alert(error.RpcError);
        }
    }
};

const get = async () => {
    if (!id.value) {
        alert('bugger off');
        return;
    }

    let client = new VotingClient(transport);

    try {
        let { response } = await client.get({
            id: id.value
        });
        get_result.value = response.vote;
    } catch (error: unknown) {
        if (error && typeof error === 'object' && "RpcError" in error) {
            alert(error.RpcError);
        }
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
        <p>{{ vote_result }}</p>
    </form>
    <hr />
    <div>
        <button @click="list">List</button>
        <div>
            {{ list_result }}
        </div>
    </div>
    <hr />
    <form @submit.prevent="get">
        <button @click="get">Get</button>
        <div>
            <input v-model="id" type="number" />
        </div>
        <div>
            {{ get_result }}
        </div>
    </form>
</template>
